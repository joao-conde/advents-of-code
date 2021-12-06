export const range = (a: number, b?: number, step = 1): number[] => {
    const size = Math.ceil(b !== undefined ? (b - a) / step : a / step);
    const offset = b !== undefined ? a : 0;
    return [...Array(size).keys()].map(i => offset + i * step);
};

export const scan = <T, X>(xs: Array<X>, seed: T, fn: (state: T, next: X) => T): T[] =>
    xs.reduce((states, x) => states.concat([fn(states[states.length - 1], x)]), [seed]);

export const sum = (xs: number[]): number => xs.reduce((sum, x) => sum + x, 0);
