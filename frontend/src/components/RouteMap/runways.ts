import type Map from "esri/Map";
import type GraphicsLayer from "esri/layers/GraphicsLayer";
import type { Airport } from '../MainPanel/RouteInfo/types';
import type { SimpleLineSymbol, TextSymbol } from 'esri/symbols';
import type { Point, Polyline } from 'esri/geometry';
import { loadModules } from "esri-loader";
import Layer from './layer';

export class AirportRunways extends Layer {
    static async initAsync(map: Map): Promise<AirportRunways> {
        const [GraphicsLayer] = await loadModules([
            "esri/layers/GraphicsLayer",
        ]);

        const layer: GraphicsLayer = new GraphicsLayer({ minScale: 200_000 });

        return new AirportRunways(map, layer);
    }

    async draw(airport: Airport, textColor: string) {
        if (!airport.runways.length) return;

        const [
            SimpleLineSymbol,
            Point,
            Polyline,
            TextSymbol,
            Graphic,
        ] = await loadModules([
            "esri/symbols/SimpleLineSymbol",
            "esri/geometry/Point",
            "esri/geometry/Polyline",
            "esri/symbols/TextSymbol",
            "esri/Graphic",
        ]);

        const runwaySymbol: SimpleLineSymbol = new SimpleLineSymbol({ width: 3, color: "black" });

        for (const runway of airport.runways) {
            if (!runway.heMarker || !runway.leMarker) continue;

            const hePos: Point = new Point({
                x: runway.heMarker.position.longitudeDeg,
                y: runway.heMarker.position.latitudeDeg,
            });

            const lePos: Point = new Point({
                x: runway.leMarker.position.longitudeDeg,
                y: runway.leMarker.position.latitudeDeg,
            });

            const runwayLine: Polyline = new Polyline({
                paths: [
                    [hePos.x, hePos.y],
                    [lePos.x, lePos.y],
                ],
            });

            this.layer.graphics.add(new Graphic(runwayLine, runwaySymbol));

            let nameProps = {
                color: textColor,
                text: runway.heMarker.name,
                angle: angleFromPos(hePos, lePos),
                yoffset: -10,
                font: { size: 8, family: "sans-serif" },
            };

            const heText: TextSymbol = new TextSymbol(nameProps);
            this.layer.graphics.add(new Graphic(hePos, heText));

            nameProps.text = runway.leMarker.name;
            nameProps.angle += 180;

            const leText: TextSymbol = new TextSymbol(nameProps);
            this.layer.graphics.add(new Graphic(lePos, leText));
        }
    }
}

interface Pos {
    x: number,
    y: number
}

// https://stackoverflow.com/a/18738281
function angleFromPos(start: Pos, end: Pos): number {
    const startRad = {
        x: toRad(start.x),
        y: toRad(start.y),
    };

    const endRad = {
        x: toRad(end.x),
        y: toRad(end.y),
    };

    const deltaLon = endRad.x - startRad.x;

    const y = Math.sin(deltaLon) * Math.cos(endRad.y);
    const x =
        Math.cos(startRad.y) * Math.sin(endRad.y) -
        Math.sin(startRad.y) * Math.cos(endRad.y) * Math.cos(deltaLon);

    return (toDeg(Math.atan2(y, x)) + 360) % 360;
}

function toRad(ang: number): number {
    return ang * (Math.PI / 180);
}

function toDeg(ang: number): number {
    return ang * (180 / Math.PI);
}