export const range = (a: number, b?: number, step = 1): number[] => {
    const size = Math.ceil(b !== undefined ? (b - a) / step : a / step);
    const offset = b !== undefined ? a : 0;
    return [...Array(size).keys()].map(i => offset + i * step);
};

export const scan = <T, X>(xs: Iterable<X>, seed: T, fn: (state: T, next: X) => T): T[] => {
    const states = [seed];
    for (const x of xs) {
        seed = fn(seed, x);
        states.push(seed);
    }
    return states;
};

export const sum = (xs: number[]): number => xs.reduce((sum, x) => sum + x, 0);
