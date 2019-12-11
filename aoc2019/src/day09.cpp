#include <fstream>
#include <functional>
#include <iostream>
#include <queue>
#include <vector>
#include <unordered_map>

using namespace std;

class IntcodeProgram{
    private:
        long long int pc; //program counter
        bool halt;
        vector<long long int> code;
        unordered_map<int, int> opcodePCOffset;
        unordered_map<int, function<long long int (int, int)>> opcodeBinFun;
        queue<long long int> inputs;

    public:       
        IntcodeProgram(vector<long long int> intcode, int extend = 0){
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

            this->opcodeBinFun[1] = [](long long int x, long long int y){return x + y;};
            this->opcodeBinFun[2] = [](long long int x, long long int y){return x * y;};

            for(int i = 0; i < extend; i++) this->code.push_back(0);
        }

        void setHalt() { this->halt = true; }
 
        void offsetPC(long long int offset){ this->pc += offset; }
 
        void jump(long long int pos) { this->pc = pos; }
 
        void processBinOpcode(long long int x, long long int y, long long int pos, function<long long int (int, int)> binFun, long long int offset){
            this->code[pos] = binFun(x, y); 
        }
 
        void consumeInput(long long int pos){
            this->code[pos] = this->inputs.front();
            this->inputs.pop();
        }

        bool halted(){ return this->halt; }

        vector<long long int> getParameterModes(long long int opcode){
            vector<long long int> modes; //(opcode, arg1, arg2, arg3)
            modes.push_back(opcode % 100);
            modes.push_back(opcode / 100 % 10);
            modes.push_back(opcode / 1000 % 10);
            modes.push_back(opcode / 10000 % 10);
            return modes;
        }

        long long int get(long long int position){ return this->code[position]; }

        long long int getArgValue(long long int pc, long long int argN, long long int mode){
            switch(mode){
                case 0: return code.at(code.at(pc + argN));
                case 1: return code.at(pc + argN);
            }
            return -1;
        }

        long long int execute(){
            while(pc < code.size() && !halt){
                vector<long long int> modes = getParameterModes(code[pc]);
                long long int opcode = modes[0], arg1, arg2, arg3, tmp;
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

        long long int execute(vector<long long int> inputs){
            for(long long int x: inputs) this->inputs.push(x);
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

    IntcodeProgram program(intcode, intcode.size());
    cout << program.execute({1}) << endl;
}