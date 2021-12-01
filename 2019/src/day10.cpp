#include <algorithm>
#include <cmath>
#include <fstream>
#include <iostream>
#include <vector>
#include <string>
#include <unordered_map>

using namespace std;

typedef unordered_map<double, pair<int, int>> doubleIntPairMap;

double angleBetweenVectors(int x1, int y1, int x2, int y2) {
    pair<double, double> vector = make_pair(x1 - x2, y1 - y2);
    double vectorNorm = sqrt(vector.first * vector.first + vector.second * vector.second);
    pair<double, double> normalizedVector = make_pair(vector.first / vectorNorm, vector.second / vectorNorm);

    // cos(alpha) = a·b / |a|·|b| ----> b = (0, -1)
    // becomes y of vec a dividing by 1 (unit vectors)
    // alpha = acos(-a.y / 1) = acos(a.y)
    double angle = round(acos(-normalizedVector.second) * 1000) / 1000;
    return (normalizedVector.first < 0 ? 2 * M_PI - angle : angle);
}

doubleIntPairMap getVisibleAsteroids(const vector<string> &map, int x, int y) {
    // angles of lines of sight vectors from (x, y) that intersect another asteroid
    doubleIntPairMap asteroidAngles;
    for (int i = 0; i < map.size(); i++) {
        for (int j = 0; j < map[i].size(); j++) {
            if (i == y && j == x) continue;
            if (map[i][j] == '.') continue;
            asteroidAngles[angleBetweenVectors(j, i, x, y)] = make_pair(j, i);
        }
    }
    return asteroidAngles;
}

int main() {
    vector<string> map;
    ifstream input("input/day10");
    string line;
    while (getline(input, line)) map.push_back(line);

    int maxAsteroids = 0;
    pair<int, int> station;
    for (int i = 0; i < map.size(); i++)
        for (int j = 0; j < map.size(); j++)
            if (map[i][j] != '.') {
                doubleIntPairMap asteroidAngles = getVisibleAsteroids(map, j, i);
                if (asteroidAngles.size() > maxAsteroids) {
                    maxAsteroids = asteroidAngles.size();
                    station = make_pair(j, i);
                }
            }
    cout << "Part1: Maximum visible asteroids are " << maxAsteroids << endl;

    int destroyed = 0, ax, ay;
    while (destroyed < 200) {
        doubleIntPairMap asteroidAngles = getVisibleAsteroids(map, station.first, station.second);
        vector<double> angles;
        transform(asteroidAngles.begin(), asteroidAngles.end(), back_inserter(angles), [](auto kv){return kv.first;});
        sort(angles.begin(), angles.end());
        for (int i = 0; i < angles.size(); i++) {
            ax = asteroidAngles[angles[i]].first;
            ay = asteroidAngles[angles[i]].second;
            map[ax][ay] = '.';
            destroyed++;
            if (destroyed == 200) break;
        }
    }
    cout << "Part2: 200th asteroid destroyed was at (" << ax << ", " << ay << ") = " << ax * 100 + ay << endl;
}
