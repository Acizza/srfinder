/** Returns true if the given object has any properties defined. */
export function hasAnyValues<T extends object>(object: T): boolean {
    return Object.values(object).some((val) => val !== undefined);
}

/** Returns the given object if any properties are specified, and undefined otherwise. */
export function trimObject<T extends object>(object: T): T | undefined {
    return hasAnyValues(object) ? object : undefined;
}