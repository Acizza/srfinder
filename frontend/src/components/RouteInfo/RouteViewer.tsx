import * as React from 'react';
import type { Route, Airport } from '../../types/route';
import PerfectScrollbar from 'react-perfect-scrollbar';
import './RouteViewer.css';

interface RouteCallbacks {
    onClick?: (route: Route) => void,
    onAirportClick?: (departure: Airport) => void,
    onHover?: (route: Route) => void,
}

interface Props extends RouteCallbacks {
    routes: Route[],
    isLoading: boolean,
    hide: boolean,
}

interface State {
    hovered?: Route,
}

class RouteViewer extends React.Component<Props, State> {
    state: State = {};

    private readonly style: React.CSSProperties | undefined = this.props.hide ? {
        animation: "open-route-table",
        animationDuration: "250ms",
        animationFillMode: "forwards",
    } : undefined;

    private onRouteHover = (route: Route) => {
        this.setState({ hovered: route });
        this.props.onHover?.(route);
    };

    private renderEmptyRoutes() {
        return (
            <RouteTableWithBody>
                <span className="centered-text uppercase">
                    No routes found
                </span>
            </RouteTableWithBody>
        );
    }

    private renderLoading() {
        return (
            <RouteTableWithBody>
                <span className="centered-text uppercase">Loading</span>
            </RouteTableWithBody>
        );
    }

    private renderRoutes() {
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
            <PerfectScrollbar className="route-table-scrollbar">
                <RouteTable>{routes}</RouteTable>
            </PerfectScrollbar>
        );
    }

    render() {
        if (this.props.hide)
            return (null);

        let renderRoute: JSX.Element;

        if (this.props.isLoading) {
            renderRoute = this.renderLoading();
        } else if (this.props.routes.length === 0) {
            renderRoute = this.renderEmptyRoutes();
        } else {
            renderRoute = this.renderRoutes();
        }

        return (
            <div className="route-table-container" style={this.style}>
                {renderRoute}
            </div>
        );
    }
}

interface ChildrenProps {
    children?: JSX.Element | JSX.Element[],
}

function RouteTable(props: ChildrenProps) {
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
                {props.children}
            </tbody>
        </table>
    );
}

function RouteTableWithBody(props: ChildrenProps) {
    return (
        <React.Fragment>
            <RouteTable />
            <div className="container-body">
                {props.children}
            </div>
        </React.Fragment>
    );
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