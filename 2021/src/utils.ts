export const range = (start: number, end: number, step = 1): number[] =>
    [...Array(end - start).keys()].map(i => start + i * step).filter(x => x < end);

export const scan = <T, X>(xs: Iterable<X>, seed: T, fn: (state: T, next: X) => T): T[] => {
    const states = [seed];
    for (const x of xs) {
        seed = fn(seed, x);
        states.push(seed);
    }
    return states;
};

export const sum = (xs: number[]): number => xs.reduce((sum, x) => sum + x, 0);
