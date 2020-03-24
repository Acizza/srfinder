import React from 'react';
import Tabs from './Tabs.jsx';
import './RouteInfo.css';

class RouteInfo extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {
        const tabs = [
            { name: "filters", content: <p>TODO (1)</p> },
            { name: "runways", content: <p>TODO (2)</p> },
            { name: "freqs", content: <p>TODO (3)</p> },
        ];

        return (
            <React.Fragment>
                <Tabs tabs={tabs} />
                <RouteViewer />
            </React.Fragment>
        );
    }
}

class RouteViewer extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {
        return (
            <React.Fragment></React.Fragment>
        );
    }
}

export default RouteInfo;