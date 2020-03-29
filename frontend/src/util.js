/**
 * Test if all characters of the string match the given predicate.
 * 
 * @param {(String) => boolean} predicate 
 * @returns {boolean}
 */
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
    return this.allChars((ch) => ch.isDigit());
}

String.prototype.isAlphanumericUpper = function () {
    return this >= 'A' && this <= 'Z';
}