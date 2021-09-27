#include <cmath>
#include <fstream>
#include <iostream>
#include <vector>

#include "Intcode.cpp"

#define BLACK '.'
#define WHITE '#'

using namespace std;

struct PointHash{
    size_t operator()(const pair<int, int> &k) const{
        return (k.first * 10 + k.second);
    }
};

unordered_map<pair<int, int>, char, PointHash> buildPointColorMap(vector<int64_t> intcode, char startColor = BLACK) {
    IntcodeProgram program(intcode, intcode.size());
    unordered_map<pair<int, int>, char, PointHash> pointColorMap;
    char dirs[] = {'^', '>', 'v', '<'};

    int curDir = 0;
    pair<int, int> curPoint = make_pair(0, 0);
    pointColorMap[curPoint] = startColor;
    while (!program.halt) {
        auto pointIt = pointColorMap.find(curPoint);
        char curColor = (pointIt != pointColorMap.end() ? pointIt->second : BLACK);
        int paintWhite = program.execute({curColor == WHITE});
        int turnRight = program.execute();

        pointColorMap[curPoint] = (paintWhite ? WHITE : BLACK);

        curDir += turnRight ? 1 : -1;
        if (curDir < 0) curDir = 3;
        if (curDir > 3) curDir = 0;

        switch (dirs[curDir]) {
            case '^': curPoint.second++; break;
            case 'v': curPoint.second--; break;
            case '<': curPoint.first--; break;
            case '>': curPoint.first++; break;
        }
    }

    return pointColorMap;
}

int main() {
    vector<int64_t> intcode;
    ifstream input("input/day11");
    while (!input.eof()) {
        int64_t code;
        input >> code;
        input.ignore();
        intcode.push_back(code);
    }
    input.close();

    int areaEstimate = buildPointColorMap(intcode).size();
    cout << "Part1: The number of at least once painted panels is " << areaEstimate << endl;

    unordered_map<pair<int, int>, char, PointHash> hullID = buildPointColorMap(intcode, WHITE);
    int sideEstimate = sqrt(areaEstimate);
    cout << "Part2" << endl;
    for (int i = -sideEstimate; i < sideEstimate; i++) {
        for (int j = -sideEstimate; j < sideEstimate; j++) {
            pair<int, int> point = make_pair(i, j);
            auto pointIt = hullID.find(point);
            cout << (pointIt != hullID.end() ? pointIt->second : BLACK);
        }
        cout << endl;
    }
}
