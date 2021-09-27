#include <algorithm>
#include <iostream>
#include <fstream>
#include <vector>

#include "Intcode.cpp"

using namespace std;

vector<int64_t> readIntcode(string filePath) {
    vector<int64_t> inputVec;
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

int main() {
    vector<int64_t> intcode = readIntcode("input/day07");
    vector<int> phases = {0, 1, 2, 3, 4};

    int maxThrust = -1;
    do {
        int thrust = 0;
        for (int i = 0; i < 5; i++) {
            IntcodeProgram program(intcode);
            thrust = program.execute({phases[i], thrust});
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

        while (true) {
            outA = pA.execute({outE});
            outB = pB.execute({outA});
            outC = pC.execute({outB});
            outD = pD.execute({outC});
            outE = pE.execute({outD});

            if (pA.halt || pB.halt || pC.halt || pD.halt || pE.halt)
                break;
            else
                lastEOut = max(lastEOut, outE);
        }
        maxThrust = max(maxThrust, lastEOut);
    } while (next_permutation(phases.begin(), phases.end()));

    cout << "Part2: Maximum feedback loop thrust is " << maxThrust << endl;
}
