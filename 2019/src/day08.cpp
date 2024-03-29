#include <algorithm>
#include <iostream>
#include <fstream>
#include <numeric>
#include <string>
#include <vector>

using namespace std;

int getLayerValCnt(const vector<vector<int>> &layer, int val) {
    vector<int> rowCnts;
    transform(layer.begin(), layer.end(), back_inserter(rowCnts), [&val](vector<int> row){return count(row.begin(), row.end(), val);});
    return accumulate(rowCnts.begin(), rowCnts.end(), 0, [](int acc, int x){return acc + x;});
}

int computePixelColor(const vector<vector<vector<int>>> &image, int w, int h) {
    for (int i = 0; i < image.size(); i++)
        if (image[i][w][h] != 2) return image[i][w][h];
    return 0;
}

int main() {
    ifstream input("input/day08");
    int width = 25, height = 6;
    vector<vector<vector<int>>> image;
    while (input.peek() != EOF) {
        vector<vector<int>> layer;
        for (int i = 0; i < height; i++) {
            vector<int> row;
            for (int j = 0; j < width; j++) {
                char pixel;
                input >> pixel;
                row.push_back(stoi(&pixel));
            }
            layer.push_back(row);
        }
        image.push_back(layer);
    }
    input.close();

    vector<int> zeroCnt;
    transform(image.begin(), image.end(), back_inserter(zeroCnt), [](vector<vector<int>> layer){return getLayerValCnt(layer, 0);});
    int fewestZeroLayerIdx = distance(zeroCnt.begin(), min_element(zeroCnt.begin(), zeroCnt.end()));
    cout << "Part1: " << getLayerValCnt(image[fewestZeroLayerIdx], 1) * getLayerValCnt(image[fewestZeroLayerIdx], 2) << endl;

    cout << "Part2:" << endl;
    for (int i = 0; i < height; i++) {
        for (int j = 0; j < width; j++)
            cout << (computePixelColor(image, i, j) ? "#" : " ");
        cout << endl;
    }
}
