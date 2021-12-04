export const sum = (xs: number[]): number => xs.reduce((sum, x) => sum + x, 0);

export const range = (start: number, end: number, step = 1): number[] =>
    [...Array(end - start).keys()].map(i => start + i * step).filter(x => x < end);
