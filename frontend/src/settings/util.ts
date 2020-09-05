interface StorageObject {
    [key: string]: string;
}

export function tryFetch<T extends StorageObject>(name: string, refObject: T): any | undefined {
    const stored = localStorage.getItem(name);

    // Check that the stored value is actually valid
    if (!stored || !Object.values(refObject).includes(stored)) {
        return undefined;
    }

    return stored as unknown as T;
}