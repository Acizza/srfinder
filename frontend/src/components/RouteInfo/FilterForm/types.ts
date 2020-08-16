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

export function okOrUndefined<T, E>(value: Result<T, E>): T | undefined {
  switch (value.kind) {
    case "ok":
      return value.value;
    case "err":
      return undefined;
  }
}

export interface FindRoutesQuery {
  speed: ParsedSpeed;
  departure?: ParsedAirportFilters;
  arrival?: ParsedAirportFilters;
  timeDist?: any;
}

export interface ParsedSpeed {
  value: number,
  type: SpeedType,
}

export const enum SpeedType {
  Mach = "mach",
  Knots = "knots",
}

export interface ParsedAirportFilters {
  icao?: string;
  type?: AirportType;
  length?: ParsedRunwayLength;
  countries?: string[];
}

export interface AirportTypes {
  unknown: string,
  large_airport: string,
  medium_airport: string,
  small_airport: string,
  closed: string,
  heliport: string,
  seaplane_base: string,
}

export type AirportType = keyof AirportTypes;

export interface ParsedRunwayLength {
  value: number,
  selector: LengthSelector
}

export const enum LengthSelector {
  LessThan = "lt",
  Equal = "eq",
  GreaterThan = "gt",
}