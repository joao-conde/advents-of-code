#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

vector<int> readInputInCppOmegaLuL(string filePath) {
    vector<int> inputVec;
    ifstream input(filePath);
    while (!input.eof()) {
        int code;
        input >> code;
        input.ignore();
        inputVec.push_back(code);
    }
    input.close();
    return inputVec;
}

int computeOutput(vector<int> intcode) {
    int i = 0, halt = 0;
    while (!halt && i < intcode.size()) {
        switch (intcode[i]) {
            case 1:
                intcode[intcode[i+3]] = intcode[intcode[i+1]] + intcode[intcode[i+2]];
                break;
            case 2:
                intcode[intcode[i+3]] = intcode[intcode[i+1]] * intcode[intcode[i+2]];
                break;
            case 99:
                halt = 1;
                break;
            default:
                halt = 1;
                break;
        }
        i+=4;
    }
    return intcode[0];
}

int main() {
    vector<int> intcode = readInputInCppOmegaLuL("input/day02");
    intcode[1] = 12;
    intcode[2] = 2;
    cout << "Part1 - " << computeOutput(intcode) << endl;

    int output = 19690720;
    for (int noun = 0; noun <= 99; noun++) {
        for (int verb = 0; verb <= 99; verb++) {
            intcode[1] = noun;
            intcode[2] = verb;
            if (computeOutput(intcode) == output) {
                cout << "Part2 - " << 100 * noun + verb << endl;
                break;
            }
        }
    }
}
