#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>
#include <unordered_map>

using namespace std;

struct Hash{
    size_t operator()(const pair<int, int> &k) const {
        return (k.first * 10 + k.second);
    }
};
typedef unordered_map<pair<int, int>, int, Hash> coordsDistanceMap;

string getMove(string &wire) {
    size_t pos = 0;
    string move;
    pos = wire.find(",");
    if (pos != string::npos) {
        move = wire.substr(0, pos);
        wire.erase(0, pos + 1);
    } else if (!wire.empty()) {
        move = wire;
        wire = "";
    }
    return move;
}

coordsDistanceMap computeCoords(string wire) {
    coordsDistanceMap map;
    int x = 0, y = 0, pathLength = 0;
    while (!wire.empty()) {
        string move = getMove(wire);
        char dir = move[0];
        move.erase(0, 1);
        switch (dir) {
            case 'R':
                for (int i = 1; i <= stoi(move); i++) map.insert(make_pair(make_pair(x + i, y), pathLength + i));
                x += stoi(move);
                break;
            case 'L':
                for (int i = 1; i <= stoi(move); i++) map.insert(make_pair(make_pair(x - i, y), pathLength + i));
                x -= stoi(move);
                break;
            case 'U':
                for (int i = 1; i <= stoi(move); i++) map.insert(make_pair(make_pair(x, y + i), pathLength + i));
                y += stoi(move);
                break;
            case 'D':
                for (int i = 1; i <= stoi(move); i++) map.insert(make_pair(make_pair(x, y - i), pathLength + i));
                y -= stoi(move);
                break;
            default:
                break;
        }
        pathLength += stoi(move);
    }
    return map;
}

int main() {
    ifstream input("input/day03");
    string wire1, wire2;
    getline(input, wire1);
    getline(input, wire2);

    coordsDistanceMap wire1Coords = computeCoords(wire1), wire2Coords = computeCoords(wire2);

    vector<pair<int, int>> wire1Keys, wire2Keys, intersection;
    for (auto kv: wire1Coords) wire1Keys.push_back(kv.first);
    for (auto kv: wire2Coords) wire2Keys.push_back(kv.first);

    sort(wire1Keys.begin(), wire1Keys.end());
    sort(wire2Keys.begin(), wire2Keys.end());
    set_intersection(wire1Keys.begin(), wire1Keys.end(), wire2Keys.begin(), wire2Keys.end(), back_inserter(intersection));

    vector<int> manhattanDistances;
    transform(intersection.begin(), intersection.end(), back_inserter(manhattanDistances), [](pair<int, int> coords){return abs(coords.first) + abs(coords.second);});
    cout << "Part1 - Min Manhattan Distance: " << *min_element(manhattanDistances.begin(), manhattanDistances.end()) << endl;

    vector<int> combinedPathLength;
    transform(intersection.begin(), intersection.end(), back_inserter(combinedPathLength), [&wire1Coords, &wire2Coords](pair<int, int> coords){return wire1Coords[coords] + wire2Coords[coords];});
    cout << "Part2 - Min combined wire path length: " << *min_element(combinedPathLength.begin(), combinedPathLength.end()) << endl;
}
