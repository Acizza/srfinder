export const enum SpeedType {
  Mach = "mach",
  Knots = "knots",
}

export class Speed {
  constructor(public value: number, public type: SpeedType) { }

  static parse(value: string): Speed | null {
    const num = Number(value);

    if (Number.isNaN(num))
      return null;

    const type = Speed.isWholeNumber(num) ? SpeedType.Knots : SpeedType.Mach;

    return new Speed(num, type);
  }

  private static isWholeNumber(value: number): boolean {
    return value % 1 === 0;
  }

  toJSON(): { value: number, type: SpeedType } {
    return {
      value: this.value,
      type: this.type,
    };
  }
}

export interface AirportFiltersInput {
  icao: string;
  type: AirportType;
  length: string;
  countries: string;
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