import { writable } from "svelte/store";

export enum Theme {
    Dark = "dark",
    Light = "light",
}

export const currentTheme = writable(detectPreferredTheme());

function detectPreferredTheme(): Theme {
    const stored = localStorage.getItem("theme");

    if (!stored) {
        return window.matchMedia("(prefers-color-scheme: dark)").matches
            ? Theme.Dark
            : Theme.Light;
    }

    // Check that the stored theme value is actually valid
    if (!Object.values(Theme).includes(stored as Theme)) {
        return Theme.Dark;
    }

    return stored as Theme;
}

export function applyTheme(theme: Theme, save: boolean) {
    document.documentElement.setAttribute("data-theme", theme);

    if (save)
        localStorage.setItem("theme", theme);

    currentTheme.set(theme);
}
