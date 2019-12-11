#include <fstream>
#include <functional>
#include <iostream>
#include <queue>
#include <vector>
#include <unordered_map>

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

int main(){
    vector<long long int> intcode;
    ifstream input("../res/day09");
    while(!input.eof()){
        long long int code;
        input >> code;
        input.ignore();
        intcode.push_back(code);
    }
    input.close();

    IntcodeProgram program(intcode, 2 * intcode.size());
    cout << program.execute({1}) << endl;
}