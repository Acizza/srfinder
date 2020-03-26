import React from 'react';
import RouteMap from './RouteMap.jsx';
import RouteInfo from './RouteInfo.jsx';
import './App.css';
import 'react-perfect-scrollbar/dist/css/styles.css';

class App extends React.Component {
    constructor(props) {
        super(props);
    }

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