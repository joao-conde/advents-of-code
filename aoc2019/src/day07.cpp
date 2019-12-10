#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>

#include "IntcodeProgram/IntcodeProgram.h"

using namespace std;

vector<int> readIntcode(string filePath){
    vector<int> inputVec;
    ifstream input(filePath);
    while(!input.eof()){
        int code;
        input >> code;
        input.ignore();
        inputVec.push_back(code);
    }
    input.close();
    return inputVec;
}

int main(){
    vector<int> intcode = readIntcode("../res/day07");
    vector<int> phases = {0, 1, 2, 3, 4};
    
    int maxThrust = -1;
    do {
        int thrust = 0;
        for(int i = 0; i < 5; i++){
            queue<int> inputs; inputs.push(phases[i]); inputs.push(thrust);
            IntcodeProgram program(intcode, inputs);
            thrust = program.execute();
        }
        maxThrust = max(maxThrust, thrust);
    } while (next_permutation(phases.begin(), phases.end()));
    
    cout << "Part1: Maximum thrust is " << maxThrust << endl;

    
}