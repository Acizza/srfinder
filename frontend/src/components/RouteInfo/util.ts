/** Returns the given object if any properties are specified, and undefined otherwise. */
export function trimObject<T extends object>(object: T): T | undefined {
    const hasAnyValue = Object.values(object).some((val) => val !== undefined);
    return hasAnyValue ? object : undefined;
}