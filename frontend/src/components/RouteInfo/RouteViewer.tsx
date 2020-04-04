import * as React from 'react';
import { Route } from './RouteInfo';
import './RouteViewer.css';

interface Props {
    routes: Route[],
}

function RouteViewer(props: Props) {
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

export default RouteViewer;