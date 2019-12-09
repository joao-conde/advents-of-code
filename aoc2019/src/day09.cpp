#include <fstream>
#include <iostream>
#include <vector>
#include <queue>

using namespace std;

class IntcodePC{
    public:
        bool halt;
        long long int relativeBase;
        vector<long long int> program;

        IntcodePC(vector<long long int> program, int extension){
            this->halt = false;
            this->relativeBase = 0;
            this->program = program;
            for(int i = 0; i < extension; i++) program.push_back(0);
        }

        vector<long long int> getParameterModes(long long int opcode){
            vector<long long int> modes; //(opcode, arg1, arg2, arg3)
            modes.push_back(opcode % 100);
            modes.push_back(opcode / 100 % 10);
            modes.push_back(opcode / 1000 % 10);
            modes.push_back(opcode / 10000 % 10);
            return modes;
        }

        long long int compute(queue<long long int> inputs){
            int i = 0;
            while(!halt && i < program.size()){
                long long int arg1, arg2, arg3;
                vector<long long int> modes = getParameterModes(program[i]);
                try{
                    switch(modes[1]){
                        case 0:
                            arg1 = program.at(program.at(i+1));
                            break;
                        case 1:
                            arg1 = program.at(i+1);
                            break;
                        case 2:
                            arg1 = program.at(program.at(i+1) + relativeBase);
                            break;
                    }

                    switch(modes[2]){
                        case 0:
                            arg2 = program.at(program.at(i+2));
                            break;
                        case 1:
                            arg2 = program.at(i+2);
                            break;
                        case 2:
                            arg2 = program.at(program.at(i+2) + relativeBase);
                            break;
                    }

                    switch(modes[3]){
                        case 0:
                            arg3 = program.at(program.at(i+3));
                            break;
                        case 1:
                            arg3 = program.at(i+3);
                            break;
                        case 2:
                            arg3 = program.at(program.at(i+3) + relativeBase);
                            break;
                    }
                } catch(const out_of_range &e){} //no more args

                switch(modes[0]){
                    case 1:
                        program[program[i+3]] = arg1 + arg2;
                        i += 4;
                        break;
                    case 2:
                        program[program[i+3]] = arg1 * arg2;
                        i += 4;
                        break;
                    case 3:
                        program[program[i+1]] = inputs.front();
                        inputs.pop();
                        i += 2;
                        break;
                    case 4:
                        return arg1;
                    case 5:
                        if(arg1) i = arg2; else i += 3;
                        break;
                    case 6:
                        if(!arg1) i = arg2; else i += 3;
                        break;
                    case 7:
                        program[program[i+3]] = arg1 < arg2;
                        i += 4;
                        break;
                    case 8:
                        program[program[i+3]] = arg1 == arg2;
                        i += 4;
                        break;
                    case 9:
                        relativeBase += arg1;
                        i += 2;
                        break;
                    case 99:
                        halt = true;
                        break;
                    default:
                        halt = true;
                        break;
                }
            }  
            return -1;
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

    IntcodePC program(intcode, 4 * intcode.size());

    queue<long long int> q;
    q.push(1);
    cout << program.compute(q) << endl;
}