import * as React from 'react';
import Tabs from './Tabs';
import FilterForm, { FilterFormSubmitEvent } from './FilterForm/FilterForm';
import './RouteInfo.css';

interface RouteInfoState {
    routes: RouteProps[],
}

class RouteInfo extends React.Component<{}, RouteInfoState> {
    state: RouteInfoState = {
        routes: [
            { from: "rjaa", to: "ksfo", time: "10:15" },
            { from: "klax", to: "klas", time: "1:38" }
        ],
    };

    // TODO: temporary
    onFilterSubmission = (event: FilterFormSubmitEvent) => {
        this.setState({
            routes: [
                { from: "vhhh", to: "ksmf", time: "15:15" },
                { from: "kdfw", to: "ksea", time: "3:38" }
            ],
        });

        event.preventDefault();
    }

    render() {
        const tabs = [
            { name: "filters", content: <FilterForm className="filter-form" onSubmit={this.onFilterSubmission} /> },
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

interface RouteViewerProps {
    routes: RouteProps[],
}

function RouteViewer(props: RouteViewerProps) {
    const routes = props.routes.map((route, i) => (
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

interface RouteProps {
    from: string,
    to: string,
    time: string,
}

function Route(props: RouteProps) {
    return (
        <tr>
            <td>{props.from}</td>
            <td>{props.to}</td>
            <td>{props.time}</td>
        </tr>
    );
}

export default RouteInfo;