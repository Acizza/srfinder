import * as React from 'react';
import { setDefaultOptions, loadModules } from 'esri-loader';
import type { Route, Airport } from '../types/route';
import './RouteMap.css';

interface Props {
    drawRoute?: Route,
}

interface State {
    isLoaded: boolean,
}

class RouteMap extends React.Component<Props, State> {
    state: State = {
        isLoaded: false,
    };

    private map: any;
    private mapRef: any = React.createRef();
    private view: any;
    private runwayLayer: any;

    componentDidMount() {
        setDefaultOptions({ css: true });

        loadModules([
            "esri/Map",
            "esri/views/MapView",
            "esri/widgets/BasemapToggle",
            "esri/layers/GraphicsLayer",
        ]).then(([
            Map,
            MapView,
            BasemapToggle,
            GraphicsLayer,
        ]) => {
            this.map = new Map({
                basemap: "gray-vector"
            });

            this.view = new MapView({
                container: this.mapRef.current,
                map: this.map,
                zoom: 2
            });

            const basemapToggle = new BasemapToggle({
                view: this.view,
                nextBasemap: "hybrid"
            });

            this.runwayLayer = new GraphicsLayer({
                minScale: 200_000
            });

            this.map.add(this.runwayLayer);
            this.view.ui.add(basemapToggle, "bottom-right");

            this.setState({ isLoaded: true });
        }).catch(err => console.error(err));
    }

    componentWillUnmount() {
        if (this.view)
            this.view.container = null;
    }

    componentDidUpdate() {
        if (!this.props.drawRoute || !this.state.isLoaded)
            return;

        this.drawCurrentRoute();
        this.drawRouteRunways(this.props.drawRoute!);
    }

    private drawCurrentRoute = () => loadModules([
        "esri/symbols/SimpleMarkerSymbol",
        "esri/symbols/SimpleLineSymbol",
        "esri/geometry/Polyline",
        "esri/geometry/geometryEngine",
        "esri/geometry/Point",
        "esri/Graphic",
    ]).then(([
        SimpleMarkerSymbol,
        SimpleLineSymbol,
        Polyline,
        geometryEngine,
        Point,
        Graphic
    ]) => {
        this.view?.graphics.removeAll();

        if (!this.props.drawRoute)
            return;

        const route = this.props.drawRoute!;

        const marker = new SimpleMarkerSymbol({
            style: "diamond",
            size: "10px"
        });

        const pointFromAirport = (airport: Airport) => new Point({
            latitude: airport.position.latitudeDeg,
            longitude: airport.position.longitudeDeg,
        });

        const depPos = pointFromAirport(route.from);
        const arrPos = pointFromAirport(route.to);

        const linePath = new Polyline({
            paths: [[depPos.x, depPos.y], [arrPos.x, arrPos.y]]
        });

        const lineSymbol = new SimpleLineSymbol({
            width: 2
        });

        const geodesicLine = geometryEngine.geodesicDensify(linePath, 10_000);

        this.view.graphics.add(new Graphic(depPos, marker));
        this.view.graphics.add(new Graphic(arrPos, marker));
        this.view.graphics.add(new Graphic(geodesicLine, lineSymbol));
    }).catch(err => console.error(err));

    private drawRouteRunways(route: Route): void {
        this.runwayLayer?.graphics.removeAll();
        this.drawAirportRunways(route.from);
        this.drawAirportRunways(route.to);
    }

    private drawAirportRunways(airport: Airport): void {
        if (airport.runways.length === 0)
            return;

        loadModules([
            "esri/symbols/SimpleLineSymbol",
            "esri/geometry/Point",
            "esri/geometry/Polyline",
            "esri/symbols/TextSymbol",
            "esri/Graphic",
        ]).then(([SimpleLineSymbol, Point, Polyline, TextSymbol, Graphic]) => {
            const runwaySymbol = new SimpleLineSymbol({
                width: 3
            });

            for (const runway of airport.runways) {
                if (!runway.heMarker || !runway.leMarker)
                    continue;

                const hePos = new Point({
                    x: runway.heMarker!.position.longitudeDeg,
                    y: runway.heMarker!.position.latitudeDeg,
                });

                const lePos = new Point({
                    x: runway.leMarker!.position.longitudeDeg,
                    y: runway.leMarker!.position.latitudeDeg,
                });

                const runwayLine = new Polyline({
                    paths: [[hePos.x, hePos.y], [lePos.x, lePos.y]]
                });

                this.runwayLayer.graphics.add(new Graphic(runwayLine, runwaySymbol));

                let nameProperties = {
                    color: "black",
                    text: runway.heMarker!.name,
                    angle: angleFromPoints(hePos, lePos),
                    yoffset: -10,
                    font: {
                        size: 8,
                        family: "sans-serif"
                    }
                };

                const heText = new TextSymbol(nameProperties);
                this.runwayLayer.graphics.add(new Graphic(hePos, heText));

                nameProperties.text = runway.leMarker!.name;
                nameProperties.angle += 180;
                const southText = new TextSymbol(nameProperties);

                this.runwayLayer.graphics.add(new Graphic(lePos, southText));
            }
        }).catch(err => console.error(err));
    }

    viewAirport = (airport: Airport) => loadModules([
        "esri/geometry/Point"
    ]).then(([
        Point
    ]) => {
        const center = new Point({
            latitude: airport.position.latitudeDeg,
            longitude: airport.position.longitudeDeg,
        });

        this.view?.goTo({
            center,
            scale: this.runwayLayer?.minScale / 2,
        });
    }).catch(err => console.error(err));

    render() {
        return (
            <div className="route-map">
                {!this.state.isLoaded && <h2 id="loading-text">Loading map</h2>}
                <div id="route-map-container" ref={this.mapRef}></div>
            </div>
        );
    }
}

interface Point {
    x: number,
    y: number,
}

// https://stackoverflow.com/a/18738281
function angleFromPoints(start: Point, end: Point): number {
    const startRad: Point = {
        x: toRad(start.x),
        y: toRad(start.y)
    };

    const endRad: Point = {
        x: toRad(end.x),
        y: toRad(end.y),
    };

    const deltaLon = endRad.x - startRad.x;

    const y = Math.sin(deltaLon) * Math.cos(endRad.y);
    const x = Math.cos(startRad.y) * Math.sin(endRad.y) - Math.sin(startRad.y)
        * Math.cos(endRad.y) * Math.cos(deltaLon);

    return (toDeg(Math.atan2(y, x)) + 360) % 360;
}

function toRad(ang: number): number {
    return ang * (Math.PI / 180);
}

function toDeg(ang: number): number {
    return ang * (180 / Math.PI);
}

export default RouteMap;