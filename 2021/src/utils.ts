export const mean = (xs: number[]): number => sum(xs) / xs.length;

export const median = (xs: number[], sort = true): number => {
    const sorted = sort ? xs.sort((a, b) => a - b) : xs;
    const i1 = Math.floor(sorted.length / 2);
    const i2 = Math.ceil(sorted.length / 2);
    return (sorted[i1] + sorted[i2]) / 2;
};

export const range = (a: number, b?: number, step = 1): number[] => {
    const size = Math.ceil(b !== undefined ? (b - a) / step : a / step);
    const offset = b !== undefined ? a : 0;
    return [...Array(size).keys()].map(i => offset + i * step);
};

export const scan = <T, X>(xs: Array<X>, seed: T, fn: (state: T, next: X) => T): T[] =>
    xs.reduce((states, x) => states.concat([fn(states[states.length - 1], x)]), [seed]);

export const sum = (xs: number[]): number => xs.reduce((sum, x) => sum + x, 0);

export const mul = (xs: number[]): number => xs.reduce((sum, x) => sum * x, 1);
