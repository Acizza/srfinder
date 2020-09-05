import { writable } from 'svelte/store';
import { get_store_value } from 'svelte/internal';
import { tryFetch } from './util';

export enum LengthUnitKind {
    Feet = "ft",
    Meters = "m",
}

// For conversions, the base unit is feet
export class LengthUnit {
    static storageName = "lengthUnit";

    static preferred(): LengthUnitKind {
        return tryFetch(LengthUnit.storageName, LengthUnitKind) || LengthUnitKind.Feet;
    }

    static save() {
        localStorage.setItem(LengthUnit.storageName, get_store_value(curLengthUnit));
    }

    static toCurrent(value: number): number {
        switch (get_store_value(curLengthUnit) as LengthUnitKind) {
            case LengthUnitKind.Feet:
                return value;
            case LengthUnitKind.Meters:
                return Math.round(value / 3.2808);
        }
    }

    static fromCurrent(value: number): number {
        switch (get_store_value(curLengthUnit) as LengthUnitKind) {
            case LengthUnitKind.Feet:
                return value;
            case LengthUnitKind.Meters:
                return Math.round(value * 3.2808);
        }
    }
}

export const curLengthUnit = writable(LengthUnit.preferred());

export enum DistanceUnitKind {
    NauticalMiles = "nm",
    Miles = "mi",
    Kilometers = "km",
}

// For conversions, the base unit is nautical miles
export class DistanceUnit {
    static storageName = "distanceUnit";

    static preferred(): DistanceUnitKind {
        return tryFetch(DistanceUnit.storageName, DistanceUnitKind) || DistanceUnitKind.NauticalMiles;
    }

    static save() {
        localStorage.setItem(DistanceUnit.storageName, get_store_value(curDistanceUnit));
    }

    static toCurrent(value: number): number {
        let result: number;

        switch (get_store_value(curDistanceUnit) as DistanceUnitKind) {
            case DistanceUnitKind.NauticalMiles:
                result = value;
                break;
            case DistanceUnitKind.Miles:
                result = value * 1.15078;
                break;
            case DistanceUnitKind.Kilometers:
                result = value * 1.852;
                break;
        }

        return Math.round(result);
    }

    static fromCurrent(value: number): number {
        let result: number;

        switch (get_store_value(curDistanceUnit) as DistanceUnitKind) {
            case DistanceUnitKind.NauticalMiles:
                result = value;
                break;
            case DistanceUnitKind.Miles:
                result = value / 1.15078;
                break;
            case DistanceUnitKind.Kilometers:
                result = value / 1.852;
                break;
        }

        return Math.round(result);
    }
}

export const curDistanceUnit = writable(DistanceUnit.preferred());