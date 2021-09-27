#include <fstream>
#include <iostream>
#include <vector>

#define INPUT_P1 1
#define INPUT_P2 5

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
    vector<int> modes; 
    modes.push_back(opcode % 100);
    modes.push_back(opcode / 100 % 10);
    modes.push_back(opcode / 1000 % 10);
    modes.push_back(opcode / 10000 % 10);
    return modes;
}

int computeOutputP1(vector<int> intcode){
    int i = 0, halt = 0;
    while(!halt && i < intcode.size()){
        int pc_inc;
        int arg1, arg2, arg3;
        vector<int> modes = getParameterModes(intcode[i]);

        try{
            arg1 = (modes[1] ? intcode.at(i+1) : intcode.at(intcode.at(i+1)));
            arg2 = (modes[2] ? intcode.at(i+2) : intcode.at(intcode.at(i+2)));
            arg3 = (modes[3] ? intcode.at(i+3) : intcode.at(intcode.at(i+3)));
        } catch(const out_of_range &e){}

        switch(modes[0]){
            case 1:
                intcode[intcode[i+3]] = arg1 + arg2;
                pc_inc = 4;
                break;
            case 2:
                intcode[intcode[i+3]] = arg1 * arg2;
                pc_inc = 4;
                break;
            case 3:
                intcode[intcode[i+1]] = INPUT_P1;
                pc_inc = 2;
                break;
            case 4:
                cout << "PROGRAM OUTPUT: " << intcode[intcode[i+1]] << endl;
                pc_inc = 2;
                break;
            case 99:
                halt = 1;
                break;
            default:
                halt = 1;
                break;
        }
        i += pc_inc;
    }
    return intcode[0];
}

int computeOutputP2(vector<int> intcode){
    int i = 0, halt = 0;
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
                intcode[intcode[i+1]] = INPUT_P2;
                i += 2;
                break;
            case 4:
                cout << "PROGRAM OUTPUT: " << intcode[intcode[i+1]] << endl;
                i += 2;
                break;
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
                halt = 1;
                break;
            default:
                halt = 1;
                break;
        }
    }
    return intcode[0];
}

int main(){
    vector<int> intcode = readIntcode("input/day05");

    cout << "----Part1 program----" << endl;
    computeOutputP1(intcode);

    cout << "----Part2 program----" << endl;
    computeOutputP2(intcode);
}
