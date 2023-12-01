#include <functional>
#include <queue>
#include <unordered_map>
#include <vector>

using namespace std;

class IntcodeProgram{
 public:
        int64_t pc, base;
        bool halt;
        vector<int64_t>  code;
        queue<int64_t>  inputs;

        IntcodeProgram(vector<int64_t> intcode, int extend = 0) {
            this->pc = 0;
            this->base = 0;
            this->halt = false;
            this->code = intcode;
            this->inputs = inputs;
            for (int i = 0; i < extend; i++) this->code.push_back(0);
        }

        vector<int64_t> getParameterModes(int64_t opcode) {
            // (opcode, arg1, arg2, arg3)
            vector<int64_t>  modes;
            modes.push_back(opcode % 100);
            modes.push_back(opcode / 100 % 10);
            modes.push_back(opcode / 1000 % 10);
            modes.push_back(opcode / 10000 % 10);
            return modes;
        }

        void addInput(int64_t input) { this->inputs.push(input); }

        void clearInputs() { this->inputs = queue<int64_t>(); }

        int64_t execute() {
            while (pc < code.size() && !halt) {
                vector<int64_t>  modes = getParameterModes(code[pc]);
                int64_t arg1, arg2, tmp;
                switch (modes[0]) {
                    case 1:  // add
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        code[modes[3] == 0 ? code[pc + 3] : code[pc + 3] + base] = arg1 + arg2;
                        pc += 4;
                        break;
                    case 2:  // multiply
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        code[modes[3] == 0 ? code[pc + 3] : code[pc + 3] + base] = arg1 * arg2;
                        pc += 4;
                        break;
                    case 3:  // input
                        code[modes[1] == 0 ? code[pc + 1] : code[pc + 1] + base] = inputs.front();
                        inputs.pop();
                        pc += 2;
                        break;
                    case 4:  // output
                        tmp = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        pc += 2;
                        return tmp;
                    case 5:  // jump-if-true
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        if (arg1)
                            pc = arg2;
                        else
                            pc += 3;
                        break;
                    case 6:  // jump-if-false
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        if (!arg1)
                            pc = arg2;
                        else
                            pc += 3;
                        break;
                    case 7:  // less-than
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        code[modes[3] == 0 ? code[pc + 3] : code[pc + 3] + base] = arg1 < arg2;
                        pc += 4;
                        break;
                    case 8:  // equals
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        code[modes[3] == 0 ? code[pc + 3] : code[pc + 3] + base] = arg1 == arg2;
                        pc += 4;
                        break;
                    case 9:  // relative base offset
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        base += arg1;
                        pc+= 2;
                        break;
                    case 99:  // halt
                    default:
                        halt = true;
                }
            }
            halt = true;
            return -1;
        }

        int64_t execute(vector<int64_t> inputs) {
            for (int64_t x: inputs) this->inputs.push(x);
            return this->execute();
        }
};
