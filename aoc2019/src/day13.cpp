#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <queue>
#include <vector>

using namespace std;

typedef long long int lli;

class IntcodeProgram{
    public:       
        lli pc, base; //program counter
        bool halt;
        vector<lli>  code;
        queue<lli>  inputs;

        IntcodeProgram(vector<lli> intcode, int extend = 0){
            this->pc = 0;
            this->base = 0;
            this->halt = false;
            this->code = intcode;
            this->inputs = inputs;
            for(int i = 0; i < extend; i++) this->code.push_back(0);
        }

        vector<lli>  getParameterModes(lli opcode){
            vector<lli>  modes; //(opcode, arg1, arg2, arg3)
            modes.push_back(opcode % 100);
            modes.push_back(opcode / 100 % 10);
            modes.push_back(opcode / 1000 % 10);
            modes.push_back(opcode / 10000 % 10);
            return modes;
        }

        lli execute(){
            while(pc < code.size() && !halt){
                vector<lli>  modes = getParameterModes(code[pc]);
                lli arg1, arg2, tmp;
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

        lli execute(vector<lli>  inputs){
            for(lli x: inputs) this->inputs.push(x);
            return this->execute();
        }
};

vector<lli> readIntcodeFile(string filePath){
    vector<lli>  intcode;
    ifstream input(filePath);
    while(!input.eof()){
        lli code;
        input >> code;
        input.ignore();
        intcode.push_back(code);
    }
    input.close();
    return intcode;
}

char getTileChar(lli tileID){
    switch(tileID){
        case 0: return ' ';
        case 1: return '#';
        case 2: return 'x';
        case 3: return '-';
        case 4: return 'O';
    }
    return ' ';
}

void printBoard(vector<vector<lli>> &board, int width, int height){
    for(lli i = 0; i < height + 1; i++){
        for(lli j = 0; j < width + 1; j++){
            cout << getTileChar(board[j][i]) << " ";
        }
        cout << endl;
    }
}

int main(){
    vector<lli> intcode = readIntcodeFile("../res/day13");

    IntcodeProgram gameCopy1(intcode, intcode.size());
    lli width = 0, height = 0;
    while(!gameCopy1.halt){
        width = max(gameCopy1.execute(), width);
        height = max(gameCopy1.execute(), height);
        gameCopy1.execute(); //disregard tileID for now
    }   

    vector<vector<lli>> board(width + 1, vector<lli>(height + 1, 0));
    IntcodeProgram gameCopy2(intcode, intcode.size());
    while(!gameCopy2.halt){
        lli x = gameCopy2.execute();
        lli y = gameCopy2.execute();
        lli tileID = gameCopy2.execute();

        if(!gameCopy2.halt) board[x][y] = tileID;
    }

    vector<int> blockRowCnt;
    auto countBlockTiles = [](vector<lli> row){return count(row.begin(), row.end(), 2);};
    transform(board.begin(), board.end(), back_inserter(blockRowCnt), countBlockTiles);
    cout << "Part1: " << accumulate(blockRowCnt.begin(), blockRowCnt.end(), 0) << " block tiles" << endl;

    printBoard(board, width, height);
}