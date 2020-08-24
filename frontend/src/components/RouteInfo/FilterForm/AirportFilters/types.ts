export interface ParsedAirportFilters {
  icao?: string;
  airportType?: AirportType;
  runwayLength?: ParsedRunwayLength;
  countries?: string[];
}

export interface ParsedSpeed {
  value: number,
  type: SpeedType,
}

export const enum SpeedType {
  Mach = "mach",
  Knots = "knots",
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
  length: number,
  selector: LengthSelector
}

export const enum LengthSelector {
  LessThan = "lt",
  Equal = "eq",
  GreaterThan = "gt",
}