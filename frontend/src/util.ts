declare global {
    interface String {
        allChars(predicate: (ch: string) => boolean): boolean,
        isCharDigit(): boolean,
        isDigits(): boolean,
        isAlphanumeric(): boolean,
    }
}

String.prototype.allChars = function (predicate) {
    if (this.length === 0)
        return false;

    for (const ch of this) {
        if (!predicate(ch))
            return false;
    }

    return true;
}

String.prototype.isCharDigit = function () {
    return this >= '0' && this <= '9';
}

String.prototype.isDigits = function () {
    return this.allChars((ch: string) => ch.isCharDigit());
}

String.prototype.isAlphanumeric = function () {
    return this.allChars(ch =>
        (ch >= '0' && ch <= '9') ||
        (ch >= 'a' && ch <= 'z') ||
        (ch >= 'A' && ch <= 'Z')
    );
}

export default {}