import * as React from 'react';
import RouteMap from './RouteMap';
import RouteInfo from './RouteInfo/RouteInfo';
import './App.css';
import 'react-perfect-scrollbar/dist/css/styles.css';

class App extends React.Component<{}, {}> {
    render() {
        return (
            <React.Fragment>
                <RouteMap />
                <RouteInfo />
            </React.Fragment>
        );
    }
}

export default App;