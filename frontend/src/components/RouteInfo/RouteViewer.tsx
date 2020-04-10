import * as React from 'react';
import { Route } from '../../types/route';
import './RouteViewer.css';

interface Props {
    routes: Route[],
}

function RouteViewer(props: Props) {
    const routes = props.routes.map((route, i) => (
        <RouteRow key={i} {...route} />
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
            <td>{route.from.icao}</td>
            <td>{route.to.icao}</td>
            <td>{timeStr}</td>
        </tr>
    );
}

export default RouteViewer;