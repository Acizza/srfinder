import type { Route, Airport } from '../MainPanel/RouteInfo/types';
import type Map from "esri/Map";
import type { Polyline, Point } from 'esri/geometry';
import type { SimpleLineSymbol, SimpleMarkerSymbol, TextSymbol } from 'esri/symbols';
import type GraphicsLayer from "esri/layers/GraphicsLayer";
import { loadModules } from 'esri-loader';
import Layer from './layer';

export class RouteLine extends Layer {
    static async initAsync(map: Map): Promise<RouteLine> {
        const [GraphicsLayer] = await loadModules([
            "esri/layers/GraphicsLayer",
        ]);

        const layer: GraphicsLayer = new GraphicsLayer();

        return new RouteLine(map, layer);
    }

    async draw(route: Route, textColor: string) {
        const [
            SimpleMarkerSymbol,
            SimpleLineSymbol,
            TextSymbol,
            Polyline,
            geometryEngine,
            Point,
            Graphic,
        ] = await loadModules([
            "esri/symbols/SimpleMarkerSymbol",
            "esri/symbols/SimpleLineSymbol",
            "esri/symbols/TextSymbol",
            "esri/geometry/Polyline",
            "esri/geometry/geometryEngine",
            "esri/geometry/Point",
            "esri/Graphic",
        ]);

        const pointFromArpt = (arpt: Airport): Point =>
            new Point({
                latitude: arpt.position.latitudeDeg,
                longitude: arpt.position.longitudeDeg,
            });

        const depPos = pointFromArpt(route.from);
        const arrPos = pointFromArpt(route.to);

        const linePath: Polyline = new Polyline({
            paths: [
                [depPos.x, depPos.y],
                [arrPos.x, arrPos.y],
            ],
        });

        const lineSymbol: SimpleLineSymbol = new SimpleLineSymbol({
            width: 2,
        });

        const geodesicLine: Polyline = geometryEngine.geodesicDensify(
            linePath,
            10_000
        );

        const posSymbol: SimpleMarkerSymbol = new SimpleMarkerSymbol({
            style: "diamond",
            size: "10px",
        });

        let nameProps = {
            color: textColor,
            text: route.from.icao,
            yoffset: 7,
            font: { size: 8, family: "sans-serif" },
        };

        const depMarker: TextSymbol = new TextSymbol(nameProps);

        nameProps.text = route.to.icao;
        const arrMarker: TextSymbol = new TextSymbol(nameProps);

        this.layer.addMany([
            // Departure / arrival position markers
            new Graphic(depPos, posSymbol),
            new Graphic(arrPos, posSymbol),
            // Route line
            new Graphic(geodesicLine, lineSymbol),
            // Departure / arrival labels
            new Graphic(depPos, depMarker),
            new Graphic(arrPos, arrMarker),
        ]);
    }
}