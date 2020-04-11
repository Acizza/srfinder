import * as React from 'react';
import RouteMap from './RouteMap';
import RouteInfo from './RouteInfo/RouteInfo';
import { Route } from '../types/route';
import './App.css';
import 'react-perfect-scrollbar/dist/css/styles.css';
import { SubmitEvent } from './RouteInfo/FilterForm/FilterForm';

interface State {
    routes: Route[],
    selectedRoute?: Route,
}

class App extends React.Component<{}, State> {
    state: State = {
        routes: [],
    };

    private onRoutesRequested = (filters: any, event: SubmitEvent) => {
        this.findRoutes(filters)
            .then(routes => this.setState({ routes }))
            .catch(err => console.error(err));

        event.preventDefault();
    }

    private onRouteSelected = (route: Route) => this.setState({ selectedRoute: route });

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

    render() {
        return (
            <React.Fragment>
                <RouteMap drawRoute={this.state.selectedRoute} />
                <RouteInfo
                    routes={this.state.routes}
                    onRoutesRequested={this.onRoutesRequested}
                    onRouteSelected={this.onRouteSelected}
                />
            </React.Fragment>
        );
    }
}

export default App;