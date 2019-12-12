#include <algorithm> 
#include <cmath>
#include <fstream>
#include <iostream>
#include <vector>
#include <string>
#include <unordered_set>

using namespace std;

int countVisibleAsteroids(const vector<string> &map, int x, int y){
    unordered_set<double> lineOfSightAngles; //angles of lines of sight vectors from (x, y) that intersect another asteroid
    for(int i = 0; i < map.size(); i++){
        for(int j = 0; j < map[i].size(); j++){
            if(i == x && j == y) continue; //self
            if(map[i][j] == '.') continue; //no asteroid

            pair<double, double> vector = make_pair(i - x, j - y);
            double vectorNorm = sqrt(vector.first * vector.first + vector.second * vector.second);            
            pair<double, double> normalizedVector = make_pair(vector.first / vectorNorm, vector.second / vectorNorm);

            // cos(alpha) = a·b / |a|·|b| ----> b = (0,1)
            // becomes y of vec a dividing by 1 (unit vectors)
            // alpha = acos(a.y / 1) = acos(a.y) 
            double angle = round(acos(normalizedVector.second) * 1000) / 1000; //round to 3 decimal place

            if (normalizedVector.first < 0) angle = 2* M_PI -angle;

            lineOfSightAngles.insert(angle);
        }
    }
    return lineOfSightAngles.size();
}

int main(){
    vector<string> map;
    ifstream input("../res/day10");
    string line;
    while(getline(input, line)) map.push_back(line);

    vector<int> visibleAsteroids;
    for(int i = 0; i < map.size(); i++) 
        for(int j = 0; j < map.size(); j++)
            if(map[i][j] != '.') visibleAsteroids.push_back(countVisibleAsteroids(map, i, j));

    cout << "Part1: Maximum visible asteroids are " << *max_element(visibleAsteroids.begin(), visibleAsteroids.end()) << endl;
}