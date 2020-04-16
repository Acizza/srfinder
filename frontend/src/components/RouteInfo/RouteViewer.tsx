import * as React from 'react';
import type { Route, Airport } from '../../types/route';
import './RouteViewer.css';

interface Props {
    routes: Route[],
    onClick?: (route: Route) => void,
    onAirportClick?: (departure: Airport) => void,
    onHover?: (route: Route) => void,
}

function RouteViewer(props: Props) {
    const routes = props.routes.map((route, i) => {
        const timeStr = `${zeroPad(route.time.hour)}:${zeroPad(route.time.minutes)}`;

        return (
            <tr key={i} onClick={() => props.onClick?.(route)} onMouseOver={() => props.onHover?.(route)}>
                <td onClick={() => props.onAirportClick?.(route.from)}>{route.from.icao}</td>
                <td onClick={() => props.onAirportClick?.(route.to)}>{route.to.icao}</td>
                <td>{timeStr}</td>
            </tr>
        );
    });

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

function zeroPad(value: number): string {
    return value < 10 ? `0${value}` : value.toString();
}

export default RouteViewer;