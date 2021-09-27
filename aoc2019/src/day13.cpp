#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <vector>

#include "Intcode.cpp"

using namespace std;

typedef long long int lli;

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

void printBoard(const vector<vector<lli>> &board, int width, int height){
    for(lli i = 0; i < height + 1; i++){
        for(lli j = 0; j < width + 1; j++){
            cout << getTileChar(board[j][i]) << " ";
        }
        cout << endl;
    }
}

int main(){
    vector<lli> intcode = readIntcodeFile("input/day13");

    IntcodeProgram gameCopy1(intcode, intcode.size());
    lli width = 0, height = 0;
    while(!gameCopy1.halt){
        width = max(gameCopy1.execute(), width);
        height = max(gameCopy1.execute(), height);
        gameCopy1.execute();
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


    intcode[0] = 2;
    IntcodeProgram gameCopy3(intcode, intcode.size());
    vector<vector<lli>> boardFree(width + 1, vector<lli>(height + 1, 0));
    lli ballX = 0, paddleX = 0, joystick = 0, score;
    while(!gameCopy3.halt){
        // clear and add joystick as input to ensure if an input is requested
        // it is the most recent position of the joystick
        gameCopy3.clearInputs();
        gameCopy3.addInput(joystick);
        lli x = gameCopy3.execute();

        gameCopy3.clearInputs();
        gameCopy3.addInput(joystick);
        lli y = gameCopy3.execute();

        gameCopy3.clearInputs();
        gameCopy3.addInput(joystick);
        lli z = gameCopy3.execute();

        if(x == -1 && y == 0){
            score = z;
            continue;
        }

        if(!gameCopy3.halt) boardFree[x][y] = z;
        if(z == 3) paddleX = x;
        if(z == 4) ballX = x;

        joystick = (ballX > paddleX ? 1 : (ballX < paddleX ? -1 : 0));
    }
    cout << "Part2: Final score " << score << endl;
}
