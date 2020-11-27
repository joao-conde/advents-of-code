#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <regex>
#include <vector>

using namespace std;

int sumOfAbsolutes(vector<int> values){
    vector<int> absolutes;
    transform(values.begin(), values.end(), back_inserter(absolutes), [](int x){return abs(x);});
    return accumulate(absolutes.begin(), absolutes.end(), 0);
}

int main(){
    ifstream input("../res/day12");
    string line, regexExp = "<x=(.*), y=(.*), z=(.*)>";
    vector<vector<int>> positions(4, vector<int>(3, 0)), velocities(4, vector<int>(3, 0));

    for(int i = 0; i < 4; i++){
        smatch match; //position 0 is full sentence, 1, 2 and 3 are coordinates
        getline(input, line);
        regex_match (line, match, regex(regexExp));
        for(int j = 0; j < 3; j++) positions[i][j] = stoi(match[j+1]);
    }

    int timestep = 1, MAX_TIMESTEP = 1000;
    while(timestep <= MAX_TIMESTEP){
        //update velocities for each pair of moons (i, j) at component k
        for(int i = 0; i < 4; i++){
            for(int j = 0; j < 4; j++){
                for(int k = 0; k < 3; k++)
                    velocities[i][k] += (positions[i][k] > positions[j][k] ? -1 : positions[i][k] < positions[j][k] ? 1 : 0);
            }
        }

        //update positions based on velocities
        for(int i = 0; i < 4; i++)
            for(int j = 0; j < 3; j++)
                positions[i][j] += velocities[i][j];

        timestep++;
    }

    int totalEnergy = 0;
    vector<int> moonsPot, moonsKin;
    transform(positions.begin(), positions.end(), back_inserter(moonsPot), [](vector<int> moonPos){return sumOfAbsolutes(moonPos);});
    transform(velocities.begin(), velocities.end(), back_inserter(moonsKin), [](vector<int> moonVel){return sumOfAbsolutes(moonVel);});
    for(int i = 0; i < 4; i++) totalEnergy += moonsPot[i] * moonsKin[i];
    cout << "Part1: " << totalEnergy << endl;
}
