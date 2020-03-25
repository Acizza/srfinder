import React from 'react';
import Tabs from './Tabs.jsx';
import './RouteInfo.css';

class RouteInfo extends React.Component {
    constructor(props) {
        super(props);

        // TODO: temporary
        this.state = {
            routes: [
                { from: "rjaa", to: "ksfo", time: "10:15" },
                { from: "klax", to: "klas", time: "1:38" }
            ],
        };
    }

    // TODO: temporary
    searchRoutes = () => {
        this.setState({
            routes: [
                { from: "vhhh", to: "ksmf", time: "15:15" },
                { from: "kdfw", to: "ksea", time: "3:38" }
            ],
        });
    }

    render() {
        const tabs = [
            { name: "filters", content: <Filters onClick={this.searchRoutes} /> },
            { name: "runways", content: <p>TODO (2)</p> },
            { name: "freqs", content: <p>TODO (3)</p> },
        ];

        return (
            <React.Fragment>
                <Tabs tabs={tabs} />
                <RouteViewer routes={this.state.routes} />
            </React.Fragment>
        );
    }
}

function Filters(props) {
    return (
        <div>
            <p>TODO (1)</p>
            <button onClick={props.onClick}>Replace Routes</button>
        </div>
    );
}

class RouteViewer extends React.Component {
    constructor(props) {
        super(props);
    }

    render() {
        const routes = this.props.routes.map((route, i) => (
            <Route key={i} from={route.from} to={route.to} time={route.time} />
        ));

        return (
            <table className="route-table">
                <thead>
                    <tr>
                        <th>From</th>
                        <th>To</th>
                        <th>Time</th>
                    </tr>
                </thead>
                <tbody>
                    {routes}
                </tbody>
            </table>
        );
    }
}

function Route(props) {
    return (
        <tr>
            <td>{props.from}</td>
            <td>{props.to}</td>
            <td>{props.time}</td>
        </tr>
    );
}

export default RouteInfo;