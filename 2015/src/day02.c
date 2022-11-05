#include <stdio.h>
#include <stdlib.h>
#include <limits.h>

int cmp_asc(const void* elem1, const void* elem2) {
    return *((int*)elem1) - *((int*)elem2);
}

int present_paper(int l, int w, int h) {
    int dimensions[] = {l * w, w * h, l * h};
    qsort(&dimensions, 3, sizeof(int), cmp_asc);

    int surface = 2 * dimensions[0] + 2 * dimensions[1] + 2 * dimensions[2];
    return surface + dimensions[0];
}

int present_ribbon(int l, int w, int h) {
    int dimensions[] = {l, w, h};
    qsort(&dimensions, 3, sizeof(int), cmp_asc);

    int volume =  dimensions[0] * dimensions[1] * dimensions[2];
    return (2 * dimensions[0] + 2 * dimensions[1]) + volume;
}

int main() {
    char* input = malloc(sizeof(char) * INT_MAX);
    FILE* file = fopen("input/day02", "r");

    int l, w, h;
    int paper = 0, ribbon = 0;
    while (fscanf(file, "%dx%dx%d\n", &l, &w, &h) != EOF) {
        paper += present_paper(l, w, h);
        ribbon += present_ribbon(l, w, h);
    }
    fclose(file);

    printf("Part1: %d\n", paper);
    printf("Part2: %d\n", ribbon);
}
