import { writable } from 'svelte/store';
import { tryFetch } from './util';
import { get_store_value } from 'svelte/internal';

export enum ThemeKind {
    Dark = "dark",
    Light = "light",
}

export class Theme {
    static storageName = "theme";

    static preferred(): ThemeKind {
        const stored = tryFetch(Theme.storageName, ThemeKind);

        if (!stored) {
            return window.matchMedia("(prefers-color-scheme: dark)").matches
                ? ThemeKind.Dark
                : ThemeKind.Light;
        }

        return stored;
    }

    static save() {
        localStorage.setItem(Theme.storageName, get_store_value(curTheme));
    }

    static apply(theme: ThemeKind) {
        document.documentElement.setAttribute("data-theme", theme);
        curTheme.set(theme);
    }
}

export const curTheme = writable(Theme.preferred());
