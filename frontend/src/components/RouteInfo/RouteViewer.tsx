import * as React from 'react';
import type { Route, Airport } from '../../types/route';
import './RouteViewer.css';

interface RouteCallbacks {
    onClick?: (route: Route) => void,
    onAirportClick?: (departure: Airport) => void,
    onHover?: (route: Route) => void,
}

interface Props extends RouteCallbacks {
    routes: Route[],
}

interface State {
    hovered?: Route,
}

class RouteViewer extends React.Component<Props, State> {
    state: State = {};

    private onRouteHover = (route: Route) => {
        this.setState({ hovered: route });
        this.props.onHover?.(route);
    };

    render() {
        const routes = this.props.routes.map((route, i) => {
            const hovered = this.state.hovered ? this.state.hovered === route : false;

            return (
                <RouteRow
                    key={i}
                    route={route}
                    hovered={hovered}
                    onClick={this.props.onClick}
                    onAirportClick={this.props.onAirportClick}
                    onHover={() => this.onRouteHover(route)}
                />
            );
        });

        return (
            <table className="route-table">
                <thead>
                    <tr>
                        <th>From</th>
                        <th>To</th>
                        <th>NM</th>
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

interface RouteRowProps extends RouteCallbacks {
    route: Route,
    hovered: boolean,
}

function RouteRow(props: RouteRowProps) {
    const route = props.route;

    const timeStr = `${zeroPad(route.time.hour)}:${zeroPad(route.time.minutes)}`;
    const classes = props.hovered ? "hovered-route" : "";

    return (
        <tr className={classes} onClick={() => props.onClick?.(props.route)} onMouseOver={() => props.onHover?.(route)}>
            <td onClick={() => props.onAirportClick?.(route.from)}>{route.from.icao}</td>
            <td onClick={() => props.onAirportClick?.(route.to)}>{route.to.icao}</td>
            <td>{Math.round(route.distance)}</td>
            <td>{timeStr}</td>
        </tr>
    );
}

function zeroPad(value: number): string {
    return value < 10 ? `0${value}` : value.toString();
}

export default RouteViewer;