#include <algorithm>
#include <fstream>
#include <iostream>
#include <numeric>
#include <vector>

#include "Intcode.cpp"

using namespace std;

vector<int64_t> readIntcodeFile(string filePath) {
    vector<int64_t>  intcode;
    ifstream input(filePath);
    while (!input.eof()) {
        int64_t code;
        input >> code;
        input.ignore();
        intcode.push_back(code);
    }
    input.close();
    return intcode;
}

char getTileChar(int64_t tileID) {
    switch (tileID) {
        case 0: return ' ';
        case 1: return '#';
        case 2: return 'x';
        case 3: return '-';
        case 4: return 'O';
    }
    return ' ';
}

void printBoard(const vector<vector<int64_t>> &board, int width, int height) {
    for (int64_t i = 0; i < height + 1; i++) {
        for (int64_t j = 0; j < width + 1; j++) {
            cout << getTileChar(board[j][i]) << " ";
        }
        cout << endl;
    }
}

int main() {
    vector<int64_t> intcode = readIntcodeFile("input/day13");

    IntcodeProgram gameCopy1(intcode, intcode.size());
    int64_t width = 0, height = 0;
    while (!gameCopy1.halt) {
        width = max(gameCopy1.execute(), width);
        height = max(gameCopy1.execute(), height);
        gameCopy1.execute();
    }

    vector<vector<int64_t>> board(width + 1, vector<int64_t>(height + 1, 0));
    IntcodeProgram gameCopy2(intcode, intcode.size());
    while (!gameCopy2.halt) {
        int64_t x = gameCopy2.execute();
        int64_t y = gameCopy2.execute();
        int64_t tileID = gameCopy2.execute();

        if (!gameCopy2.halt) board[x][y] = tileID;
    }

    vector<int> blockRowCnt;
    auto countBlockTiles = [](vector<int64_t> row){return count(row.begin(), row.end(), 2);};
    transform(board.begin(), board.end(), back_inserter(blockRowCnt), countBlockTiles);
    cout << "Part1: " << accumulate(blockRowCnt.begin(), blockRowCnt.end(), 0) << " block tiles" << endl;


    intcode[0] = 2;
    IntcodeProgram gameCopy3(intcode, intcode.size());
    vector<vector<int64_t>> boardFree(width + 1, vector<int64_t>(height + 1, 0));
    int64_t ballX = 0, paddleX = 0, joystick = 0, score;
    while (!gameCopy3.halt) {
        // clear and add joystick as input to ensure if an input is requested
        // it is the most recent position of the joystick
        gameCopy3.clearInputs();
        gameCopy3.addInput(joystick);
        int64_t x = gameCopy3.execute();

        gameCopy3.clearInputs();
        gameCopy3.addInput(joystick);
        int64_t y = gameCopy3.execute();

        gameCopy3.clearInputs();
        gameCopy3.addInput(joystick);
        int64_t z = gameCopy3.execute();

        if (x == -1 && y == 0) {
            score = z;
            continue;
        }

        if (!gameCopy3.halt) boardFree[x][y] = z;
        if (z == 3) paddleX = x;
        if (z == 4) ballX = x;

        joystick = (ballX > paddleX ? 1 : (ballX < paddleX ? -1 : 0));
    }
    cout << "Part2: Final score " << score << endl;
}
