import * as React from 'react';
import Tabs, { Tab } from './Tabs';
import FilterForm, { State as FilterFormState, SubmitEvent } from './FilterForm/FilterForm';
import './RouteInfo.css';
import { Time } from './FilterForm/TimeInput';

interface Route {
    from: string,
    to: string,
    distance: number,
    time: Time,
}

interface RouteInfoState {
    routes: Route[],
}

class RouteInfo extends React.Component<{}, RouteInfoState> {
    state: RouteInfoState = {
        routes: [],
    };

    private onFilterSubmission = (state: FilterFormState, event: SubmitEvent) => {
        this.findRoutes(state)
            .then(routes => this.setState({ routes }))
            .catch(err => console.error(err));

        event.preventDefault();
    }

    private async findRoutes(state: FilterFormState): Promise<Route[]> {
        const resp = await fetch("/search_routes", {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify(state)
        });

        if (!resp.ok)
            throw Error(`Searching for routes failed with response code: ${resp.status}`);

        const json = await resp.json();

        if (!json.routes)
            throw Error(`Malformed json response while fetching routes: ${json}`);

        return json.routes;
    }

    render() {
        const tabs: Tab[] = [
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
    routes: Route[],
}

function RouteViewer(props: RouteViewerProps) {
    const routes = props.routes.map((route, i) => (
        <RouteRow key={i} from={route.from} to={route.to} distance={route.distance} time={route.time} />
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

function RouteRow(route: Route) {
    const timeStr = `${route.time.hour}:${route.time.minutes}`;

    return (
        <tr>
            <td>{route.from}</td>
            <td>{route.to}</td>
            <td>{timeStr}</td>
        </tr>
    );
}

export default RouteInfo;