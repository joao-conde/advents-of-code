#include <algorithm>
#include <iostream>
#include <fstream>
#include <string>
#include <vector>

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

vector<int> getParameterModes(int opcode){
    vector<int> modes; //(opcode, arg1, arg2, arg3)
    modes.push_back(opcode % 100);
    modes.push_back(opcode / 100 % 10);
    modes.push_back(opcode / 1000 % 10);
    modes.push_back(opcode / 10000 % 10);
    return modes;
}

int runIntcodeProg(vector<int> intcode, int phase, int input){
    int i = 0, halt = 0, inputReq = 0;
    while(!halt && i < intcode.size()){
        int arg1, arg2, arg3;
        vector<int> modes = getParameterModes(intcode[i]);
        try{
            arg1 = (modes[1] ? intcode.at(i+1) : intcode.at(intcode.at(i+1)));
            arg2 = (modes[2] ? intcode.at(i+2) : intcode.at(intcode.at(i+2)));
            arg3 = (modes[3] ? intcode.at(i+3) : intcode.at(intcode.at(i+3)));
        } catch(const out_of_range &e){} //no more args

        switch(modes[0]){
            case 1:
                intcode[intcode[i+3]] = arg1 + arg2;
                i += 4;
                break;
            case 2:
                intcode[intcode[i+3]] = arg1 * arg2;
                i += 4;
                break;
            case 3:
                intcode[intcode[i+1]] = (inputReq == 0 ? phase : input);
                inputReq++;
                i += 2;
                break;
            case 4:
                return intcode[intcode[i+1]];
            case 5:
                if(arg1) i = arg2; else i += 3;
                break;
            case 6:
                if(!arg1) i = arg2; else i += 3;
                break;
            case 7:
                intcode[intcode[i+3]] = arg1 < arg2;
                i += 4;
                break;
            case 8:
                intcode[intcode[i+3]] = arg1 == arg2;
                i += 4;
                break;
            case 99:
                cout << "HALT" << endl;
                halt = 1;
                break;
            default:
                halt = 1;
                break;
        }
    }  
    return -1;   
}

int main(){
    vector<int> intcode = readIntcode("../res/day07");
    vector<int> phases = {0, 1, 2, 3, 4};
    
    int maxThrust = -1;
    do {
        int thrust = 0;
        for(int i = 0; i < 5; i++){
            thrust = runIntcodeProg(intcode, phases[i], thrust);
        }
        maxThrust = max(maxThrust, thrust);
    } while (next_permutation(phases.begin(), phases.end()));
    
    cout << "Part1: Maximum thrust is " << maxThrust << endl;

    
}