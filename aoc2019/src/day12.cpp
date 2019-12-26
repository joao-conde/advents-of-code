#include <fstream>
#include <iostream>
#include <regex>
#include <vector>

using namespace std;

int main(){
    ifstream input("../res/day12");
    string line, regexExp = "<x=(.*), y=(.*), z=(.*)>";
    int positions[4][3], velocities[4][3];
    memset(velocities, 0, 4 * 3 * sizeof(int));
    for(int i = 0; i < 4; i++){
        smatch match; //position 0 is full sentence, 1, 2 and 3 are coordinates
        getline(input, line);
        regex_match (line, match, regex(regexExp));
        for(int j = 0; j < 3; j++) positions[i][j] = stoi(match[j+1]);
    }

    int timestep = 0;
    while(timestep <= 10){
        cout << "TIMESTEP: " << timestep << endl;
        for(int k = 0; k < 4; k++){
            cout << positions[k][0] << ", " << positions[k][1] << ", " << positions[k][2] << endl;
            cout << velocities[k][0] << ", " << velocities[k][1] << ", " << velocities[k][2] << endl;
        }

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

}