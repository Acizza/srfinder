import * as React from 'react';
import RouteMap from './RouteMap';
import RouteInfo from './RouteInfo';
import './App.css';
import 'react-perfect-scrollbar/dist/css/styles.css';

class App extends React.Component<{}, {}> {
    render() {
        return (
            <React.Fragment>
                <div id="route-map">
                    <RouteMap />
                </div>
                <div id="route-info">
                    <RouteInfo />
                </div>
            </React.Fragment>
        );
    }
}

export default App;