import * as React from 'react';
import RouteMap from './RouteMap';
import RouteInfo from './RouteInfo/RouteInfo';
import type { Airport, Route } from '../types/route';
import './App.css';
import 'react-perfect-scrollbar/dist/css/styles.css';

interface State {
    selectedRoute?: Route,
}

class App extends React.Component<{}, State> {
    state: State = {};

    private mapRef: React.RefObject<RouteMap> = React.createRef();

    private onRouteSelected = (route: Route) => this.setState({ selectedRoute: route });
    private onAirportClick = (airport: Airport) => this.mapRef.current?.viewAirport(airport);

    render() {
        return (
            <React.Fragment>
                <RouteMap drawRoute={this.state.selectedRoute} ref={this.mapRef} />
                <RouteInfo
                    onRouteSelected={this.onRouteSelected}
                    onAirportClick={this.onAirportClick}
                />
            </React.Fragment>
        );
    }
}

export default App;