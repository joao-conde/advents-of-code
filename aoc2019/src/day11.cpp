#include <fstream>
#include <iostream>
#include <queue>
#include <unordered_map>
#include <vector>

using namespace std;

class IntcodeProgram{
    public:       
        long long int pc, base; //program counter
        bool halt;
        vector<long long int> code;
        queue<long long int> inputs;

        IntcodeProgram(vector<long long int> intcode, int extend = 0){
            this->pc = 0;
            this->base = 0;
            this->halt = false;
            this->code = intcode;
            this->inputs = inputs;
            for(int i = 0; i < extend; i++) this->code.push_back(0);
        }

        vector<long long int> getParameterModes(long long int opcode){
            vector<long long int> modes; //(opcode, arg1, arg2, arg3)
            modes.push_back(opcode % 100);
            modes.push_back(opcode / 100 % 10);
            modes.push_back(opcode / 1000 % 10);
            modes.push_back(opcode / 10000 % 10);
            return modes;
        }

        long long int execute(){
            while(pc < code.size() && !halt){
                vector<long long int> modes = getParameterModes(code[pc]);
                long long int arg1, arg2, tmp;
                switch(modes[0]){
                    case 1: //add
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        code[modes[3] == 0 ? code[pc + 3] : code[pc + 3] + base] = arg1 + arg2;
                        pc += 4;
                        break;
                    case 2: //multiply
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        code[modes[3] == 0 ? code[pc + 3] : code[pc + 3] + base] = arg1 * arg2;
                        pc += 4;
                        break;
                    case 3: //input
                        code[modes[1] == 0 ? code[pc + 1] : code[pc + 1] + base] = inputs.front();
                        inputs.pop();
                        pc += 2;
                        break;
                    case 4: //output
                        tmp = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        pc += 2;
                        return tmp;
                    case 5: //jump-if-true
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        if(arg1) pc = arg2;
                        else pc += 3;
                        break;
                    case 6: //jump-if-false
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        if(!arg1) pc = arg2;
                        else pc += 3;
                        break;
                    case 7: //less-than
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        code[modes[3] == 0 ? code[pc + 3] : code[pc + 3] + base] = arg1 < arg2;
                        pc += 4;
                        break;
                    case 8: //equals
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        arg2 = (modes[2] == 1 ? code[pc + 2] : modes[2] == 0 ? code[code[pc + 2]] : code[base + code[pc + 2]]);
                        code[modes[3] == 0 ? code[pc + 3] : code[pc + 3] + base] = arg1 == arg2;
                        pc += 4;
                        break;
                    case 9: //relative base offset
                        arg1 = (modes[1] == 1 ? code[pc + 1] : modes[1] == 0 ? code[code[pc + 1]] : code[base + code[pc + 1]]);
                        base += arg1;
                        pc+= 2;
                        break;
                    case 99: //halt
                    default:
                        halt = true;
                }
            }
            halt = true;
            return -1;
        }

        long long int execute(vector<long long int> inputs){
            for(long long int x: inputs) this->inputs.push(x);
            return this->execute();
        }
};

struct PointHash{ 
    size_t operator()(const pair<int,int> &k) const{
        return (k.first * 10 + k.second);
    }
};

int main(){
    vector<long long int> intcode;
    ifstream input("../res/day11");
    while(!input.eof()){
        long long int code;
        input >> code;
        input.ignore();
        intcode.push_back(code);
    }
    input.close();

    IntcodeProgram program(intcode, intcode.size());
    unordered_map<pair<int,int>, char, PointHash> pointColorMap; //. -> black, # -> white
    char dirs[] = {'^', '>', 'v', '<'};
    
    int curDir = 0;
    pair<int,int> curPoint = make_pair(0, 0);
    while(!program.halt){
        auto pointIt = pointColorMap.find(curPoint);
        char curColor = (pointIt != pointColorMap.end() ? pointIt->second : '.');
        int paintWhite = program.execute({curColor == '#'});
        int turnRight = program.execute();

        //paint
        pointColorMap[curPoint] = (paintWhite ? '#' : '.');

        //rotate
        if(turnRight) curDir++;
        else curDir--;

        if(curDir < 0) curDir = 3;
        if(curDir > 3) curDir = 0;

        //move
        switch(dirs[curDir]){
            case '^': curPoint.second++; break;
            case 'v': curPoint.second--; break;
            case '<': curPoint.first--; break;
            case '>': curPoint.first++; break;
        }
    }

    cout << "Part1: The number of at least once painted panels is " << pointColorMap.size() << endl;
}