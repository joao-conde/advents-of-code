export const range = (start: number, end: number, step = 1): number[] =>
    [...Array(end - start).keys()].map(i => start + i * step).filter(x => x < end);

export const scan = <T, K>(iter: Iterable<K>, accum: T, fn: (state: T, next: K) => T): T[] => {
    const states = [accum];
    for (const k of iter) {
        accum = fn(accum, k);
        states.push(accum);
    }
    return states;
};

export const sum = (xs: number[]): number => xs.reduce((sum, x) => sum + x, 0);
