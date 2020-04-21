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
    isLoadingRoutes: boolean,
    isFirstRouteFetch: boolean,
}

class RouteInfo extends React.Component<Props, State> {
    state: State = {
        routes: [],
        isLoadingRoutes: false,
        isFirstRouteFetch: true,
    };

    private onRoutesRequested = (filters: FilterFormState, event: SubmitEvent) => {
        this.setState({ isLoadingRoutes: true, isFirstRouteFetch: false });

        this.findRoutes(filters)
            .then(routes => this.setState({ routes, isLoadingRoutes: false }))
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
                <RouteViewer
                    routes={this.state.routes}
                    isLoading={this.state.isLoadingRoutes}
                    hide={this.state.isFirstRouteFetch}
                    onClick={this.onRouteSelected}
                    onAirportClick={this.props.onAirportClick}
                    onHover={this.onRouteSelected}
                />
                <Tabs tabs={tabs} />
            </div>
        );
    }
}

export default RouteInfo;