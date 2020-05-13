interface String {
    allChars(predicate: (ch: string) => boolean): boolean,
    isDigit(): boolean,
    isDigits(): boolean,
    isAlphanumericUpper(): boolean,
}

String.prototype.allChars = function (predicate) {
    for (const ch of this) {
        if (!predicate(ch))
            return false;
    }

    return true;
}

String.prototype.isDigit = function () {
    return this >= '0' && this <= '9';
}

String.prototype.isDigits = function () {
    return this.allChars((ch: string) => ch.isDigit());
}

String.prototype.isAlphanumericUpper = function () {
    return this >= 'A' && this <= 'Z';
}
