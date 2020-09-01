import type { ParsedSpeed, ParsedAirportFilters } from './FilterForm/AirportFilters/types';

export type Ok<T> = {
  kind: "ok",
  value: T,
}

export type Err<T> = {
  kind: "err",
  value: T,
}

export type Result<T, E> = Ok<T> | Err<E>;
export type InputResult = Result<string, string>;

export interface FindRoutesQuery {
  speed: ParsedSpeed;
  departure?: ParsedAirportFilters;
  arrival?: ParsedAirportFilters;
  timeDist?: ParsedTimeDist;
}

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
  [key: string]: string | undefined,
}

export interface Route {
  from: Airport,
  to: Airport,
  distance: number,
  time: Time,
}

export interface Time {
  hour: number,
  minutes: number,
}

export interface ParsedRange<T> {
  min?: T,
  max?: T,
}

export type TimeDistVariant = "time" | "dist";

export interface ParsedTimeDist {
  type: TimeDistVariant,
  value: ParsedRange<Time | number>,
}