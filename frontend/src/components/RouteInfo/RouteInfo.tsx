import * as React from 'react';
import Tabs, { Tab } from './Tabs';
import FilterForm, { State as FilterFormState, SubmitEvent } from './FilterForm/FilterForm';
import RouteViewer from './RouteViewer';
import { Route } from '../../types/route';
import './RouteInfo.css';

interface State {
    routes: Route[],
}

class RouteInfo extends React.Component<{}, State> {
    state: State = {
        routes: [],
    };

    private onFilterSubmission = (state: FilterFormState, event: SubmitEvent) => {
        this.findRoutes(state)
            .then(routes => this.setState({ routes }))
            .catch(err => console.error(err));

        event.preventDefault();
    }

    private async findRoutes(state: FilterFormState): Promise<Route[]> {
        const resp = await fetch("/search_routes", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(state)
        });

        if (!resp.ok)
            throw Error(`Searching for routes failed with response code: ${resp.status}`);

        const json = await resp.json();

        if (!json.routes)
            throw Error(`Malformed json response while fetching routes: ${json}`);

        return json.routes;
    }

    render() {
        const tabs: Tab[] = [
            { name: "filters", content: <FilterForm className="filter-form" onSubmit={this.onFilterSubmission} /> },
            { name: "runways", content: <p>TODO (2)</p> },
            { name: "freqs", content: <p>TODO (3)</p> },
        ];

        return (
            <div className="route-info">
                <Tabs tabs={tabs} />
                <RouteViewer routes={this.state.routes} />
            </div>
        );
    }
}

export default RouteInfo;