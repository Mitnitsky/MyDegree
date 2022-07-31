export function decimalAdjust(type: string, value: number, exp: number): number {
  if (typeof exp === 'undefined' || +exp === 0) {
    return Math[type](value)
  }
  value = +value
  exp = +exp
  if (isNaN(value) || !(exp % 1 === 0)) {
    return NaN
  }
  let new_value = value.toString().split('e')
  value = Math[type](+(new_value[0] + 'e' + (new_value[1] ? +new_value[1] - exp : -exp)))
  new_value = value.toString().split('e')
  return +(new_value[0] + 'e' + (new_value[1] ? +new_value[1] + exp : exp))
}

export function MathRound10(value: number, exp: number): number {
  return decimalAdjust('round', value, exp)
}

export function MathFloor10(value: number, exp: number): number {
  return decimalAdjust('floor', value, exp)
}

export function MathCeil10(value: number, exp: number): number {
  return decimalAdjust('ceil', value, exp)
}
