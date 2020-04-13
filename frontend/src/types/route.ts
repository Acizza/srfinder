import { Time } from './time';

export interface Airport {
    icao: string,
    position: CoordPos,
    runways: Runway[],
    frequencies: Frequencies,
    countryName: string,
}

export interface CoordPos {
    latitudeDeg: number,
    longitudeDeg: number,
}

export interface Runway {
    lengthFT?: number,
    widthFT?: number,
    heMarker?: RunwayMarker,
    leMarker?: RunwayMarker,
}

export interface RunwayMarker {
    name: string,
    position: CoordPos,
}

export interface Frequencies {
    atis?: string,
    arrival?: string,
    departure?: string,
    arrivalDeparture?: string,
    ground?: string,
    tower?: string,
    unicom?: string,
}

export interface Route {
    from: Airport,
    to: Airport,
    distance: number,
    time: Time,
}