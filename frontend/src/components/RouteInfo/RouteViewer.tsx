import * as React from 'react';
import { Route } from '../../types/route';
import './RouteViewer.css';

interface Props {
    routes: Route[],
    onClick?: (route: Route) => void,
}

function RouteViewer(props: Props) {
    const routes = props.routes.map((route, i) => (
        <RouteRow key={i} route={route} onClick={props.onClick} />
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
    route: Route,
    onClick?: (route: Route) => void,
}

function RouteRow(props: RouteProps) {
    const route = props.route;
    const timeStr = `${route.time.hour}:${route.time.minutes}`;

    return (
        <tr onClick={() => props.onClick?.(route)}>
            <td>{route.from.icao}</td>
            <td>{route.to.icao}</td>
            <td>{timeStr}</td>
        </tr>
    );
}

export default RouteViewer;