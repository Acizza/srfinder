import * as React from 'react';
import Tabs, { Tab } from './Tabs';
import FilterForm, { State as FilterFormState, SubmitEvent } from './FilterForm/FilterForm';
import RouteViewer from './RouteViewer';
import type { Airport, Route } from '../../types/route';
import './RouteInfo.css';

interface Props {
    onRouteSelected?: (route: Route) => void,
    onAirportClick?: (airport: Airport) => void,
}

interface State {
    routes: Route[],
    selectedRoute?: Route,
}

class RouteInfo extends React.Component<Props, State> {
    state: State = {
        routes: [],
    };

    private onRoutesRequested = (filters: FilterFormState, event: SubmitEvent) => {
        this.findRoutes(filters)
            .then(routes => this.setState({ routes }))
            .catch(err => console.error(err));

        event.preventDefault();
    }

    private async findRoutes(filters: any): Promise<Route[]> {
        const resp = await fetch("/search_routes", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(filters)
        });

        if (!resp.ok)
            throw Error(`Searching for routes failed with response code: ${resp.status}`);

        const json = await resp.json();

        if (!json.routes)
            throw Error(`Malformed json response while fetching routes: ${json}`);

        return json.routes;
    }

    private onRouteSelected = (route: Route) => {
        this.setState({ selectedRoute: route });
        this.props.onRouteSelected?.(route);
    }

    render() {
        const tabs: Tab[] = [
            { name: "filters", content: <FilterForm className="filter-form" onSubmit={this.onRoutesRequested} /> },
            { name: "runways", content: <p>TODO (2)</p> },
            { name: "freqs", content: <p>TODO (3)</p> },
        ];

        return (
            <div className="route-info">
                <Tabs tabs={tabs} />
                <RouteViewer
                    routes={this.state.routes}
                    onClick={this.onRouteSelected}
                    onAirportClick={this.props.onAirportClick}
                    onHover={this.onRouteSelected}
                />
            </div>
        );
    }
}

export default RouteInfo;