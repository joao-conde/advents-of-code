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
            vector<int> inputs = {phases[i], thrust};
            IntcodeProgram program(intcode);
            program.addInputs(inputs);
            thrust = program.execute();
        }
        maxThrust = max(maxThrust, thrust);
    } while (next_permutation(phases.begin(), phases.end()));
    
    cout << "Part1: Maximum thrust is " << maxThrust << endl;

    maxThrust = -1;
    phases = {5, 6, 7, 8, 9};
    do {
        IntcodeProgram pA(intcode), pB(intcode), pC(intcode), pD(intcode), pE(intcode);
        int outA = pA.execute({phases[0], 0});
        int outB = pB.execute({phases[1], outA});
        int outC = pC.execute({phases[2], outB});
        int outD = pD.execute({phases[3], outC});
        int outE = pE.execute({phases[4], outD});
        int lastEOut = outE;
        
        while(true){
            outA = pA.execute({outE});            
            // if(pA.halted()) break;
            outB = pB.execute({outA});
            // if(pB.halted()) break;
            outC = pC.execute({outB});
            // if(pC.halted()) break;
            outD = pD.execute({outC});
            // if(pD.halted()) break;
            outE = pE.execute({outD});
            // if(pE.halted()) break;

            if(pA.halted() || pB.halted() || pC.halted() || pD.halted() || pE.halted())
                break;
            else
                lastEOut = max(lastEOut, outE);
        }
        maxThrust = max(maxThrust, lastEOut);
    } while (next_permutation(phases.begin(), phases.end()));

    cout << "Part2: Maximum feedback loop thrust is " << maxThrust << endl;
}