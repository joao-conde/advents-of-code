#include <algorithm>
#include <iostream>
#include <fstream>
#include <functional>
#include <queue>
#include <string>
#include <unordered_map>
#include <vector>

using namespace std;

class IntcodeProgram{
    private:
        int pc; //program counter
        bool halt;
        vector<int> code;
        unordered_map<int, int> opcodePCOffset;
        unordered_map<int, function<int (int, int)>> opcodeBinFun;
        queue<int> inputs;

    public:       
        IntcodeProgram(vector<int> intcode){
            this->pc = 0;
            this->halt = false;
            this->code = intcode;
            this->inputs = inputs;

            this->opcodePCOffset[1] = 4;
            this->opcodePCOffset[2] = 4;
            this->opcodePCOffset[3] = 2;
            this->opcodePCOffset[4] = 2;
            this->opcodePCOffset[5] = 3;
            this->opcodePCOffset[6] = 3;
            this->opcodePCOffset[7] = 4;
            this->opcodePCOffset[8] = 4;

            this->opcodeBinFun[1] = [](int x, int y){return x + y;};
            this->opcodeBinFun[2] = [](int x, int y){return x * y;};
        }

        void setHalt() { this->halt = true; }
 
        void offsetPC(int offset){ this->pc += offset; }
 
        void jump(int pos) { this->pc = pos; }
 
        void processBinOpcode(int x, int y, int pos, function<int (int, int)> binFun, int offset){
            this->code[pos] = binFun(x, y); 
        }
 
        void consumeInput(int pos){
            this->code[pos] = this->inputs.front();
            this->inputs.pop();
        }

        bool halted(){ return this->halt; }

        vector<int> getParameterModes(int opcode){
            vector<int> modes; //(opcode, arg1, arg2, arg3)
            modes.push_back(opcode % 100);
            modes.push_back(opcode / 100 % 10);
            modes.push_back(opcode / 1000 % 10);
            modes.push_back(opcode / 10000 % 10);
            return modes;
        }

        int get(int position){ return this->code[position]; }

        int getArgValue(int pc, int argN, int mode){
            switch(mode){
                case 0: return code.at(code.at(pc + argN));
                case 1: return code.at(pc + argN);
            }
            return -1;
        }

        int execute(){
            while(pc < code.size() && !halt){
                vector<int> modes = getParameterModes(code[pc]);
                int opcode = modes[0], arg1, arg2, arg3, tmp;
                try{
                    arg1 = getArgValue(pc, 1, modes[1]);
                    arg2 = getArgValue(pc, 2, modes[2]);
                    arg3 = getArgValue(pc, 3, modes[3]);
                } catch(const out_of_range &e){} //no more args

                switch(modes[0]){
                    case 1: //add
                    case 2: //multiply
                        processBinOpcode(arg1, arg2, code[pc + 3], opcodeBinFun[opcode], opcodePCOffset[opcode]);
                        offsetPC(opcodePCOffset[opcode]);
                        break;
                    case 3: //input
                        consumeInput(code[pc + 1]);
                        offsetPC(opcodePCOffset[opcode]);
                        break;
                    case 4: //output
                        tmp = code[code[pc + 1]];
                        offsetPC(opcodePCOffset[opcode]);
                        return tmp;
                        // break;
                    case 5: //jump-if-true
                        if(arg1) jump(arg2);
                        else offsetPC(opcodePCOffset[opcode]);
                        break;
                    case 6: //jump-if-false
                        if(!arg1) jump(arg2);
                        else offsetPC(opcodePCOffset[opcode]);
                        break;
                    case 7: //less-than
                        code[code[pc + 3]] = arg1 < arg2;
                        offsetPC(opcodePCOffset[opcode]);
                        break;
                    case 8: //equals
                        code[code[pc + 3]] = arg1 == arg2;
                        offsetPC(opcodePCOffset[opcode]);
                        break;
                    case 99: //halt
                    default:
                        setHalt();
                }
            }
            setHalt();
            return -1;
        }

        int execute(vector<int> inputs){
            for(int x: inputs) this->inputs.push(x);
            return this->execute();
        }
};

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
        
        while(true){
            outA = pA.execute({outE});            
            outB = pB.execute({outA});
            outC = pC.execute({outB});
            outD = pD.execute({outC});
            outE = pE.execute({outD});

            if(pA.halted() || pB.halted() || pC.halted() || pD.halted() || pE.halted())
                break;
            else
                lastEOut = max(lastEOut, outE);
        }
        maxThrust = max(maxThrust, lastEOut);
    } while (next_permutation(phases.begin(), phases.end()));

    cout << "Part2: Maximum feedback loop thrust is " << maxThrust << endl;
}