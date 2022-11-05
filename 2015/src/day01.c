#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <limits.h>

int main() {
    char* input = malloc(sizeof(char) * INT_MAX);
    FILE* file = fopen("input/day01", "r");
    fscanf(file, "%s", input);
    fclose(file);

    int floor = 0, position = -1;
    int nchars = strlen(input);
    for (int i = 0; i< nchars; i++) {
        floor = floor + (input[i] == '(' ? 1 : -1);
        if (position == -1 && floor == -1) position = i + 1;
    }
    free(input);

    printf("Part1: %d\n", floor);
    printf("Part2: %d\n", position);
}
