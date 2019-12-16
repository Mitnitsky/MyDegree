export function decimalAdjust(type, value, exp) {
    if (typeof exp === 'undefined' || +exp === 0) {
        return Math[type](value);
    }
    value = +value;
    exp = +exp;
    if (isNaN(value) || !(typeof exp === 'number' && exp % 1 === 0)) {
        return NaN;
    }
    value = value.toString().split('e');
    value = Math[type](+(value[0] + 'e' + (value[1] ? (+value[1] - exp) : -exp)));
    value = value.toString().split('e');
    return +(value[0] + 'e' + (value[1] ? (+value[1] + exp) : exp));
}

export function MathRound10(value, exp) {
    return decimalAdjust('round', value, exp);
}

export function MathFloor10(value, exp) {
    return decimalAdjust('floor', value, exp);
}

export function MathCeil10(value, exp) {
    return decimalAdjust('ceil', value, exp);
}
