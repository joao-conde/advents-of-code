export const range = (start: number, end: number, step = 1): number[] =>
    [...Array(end - start).keys()].map(i => start + i * step).filter(x => x < end);

export const scan = <T, K>(iter: Iterable<K>, seed: T, fn: (state: T, next: K) => T): T[] => {
    const states = [seed];
    for (const k of iter) {
        seed = fn(seed, k);
        states.push(seed);
    }
    return states;
};

export const sum = (xs: number[]): number => xs.reduce((sum, x) => sum + x, 0);
