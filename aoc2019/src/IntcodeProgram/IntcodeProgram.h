#pragma once

#include <functional>
#include <unordered_map>
#include <vector>
#include <queue>

using namespace std;

class IntcodeProgram{
    private:
        int pc; //program counter
        bool halt;
        vector<int> code;
        queue<int> inputs;
        unordered_map<int, int> opcodePCOffset;
        unordered_map<int, function<int (int, int)>> opcodeBinFun;

    public:
        IntcodeProgram(vector<int> intcode, queue<int> inputs);
        int get(int position);
        bool halted();
        void setHalt();
        void offsetPC(int offset);
        void jump(int pos);
        void processBinOpcode(int x, int y, int pos, function<int (int, int)> binFun, int offset);
        void consumeInput(int pos);
        vector<int> getParameterModes(int opcode);
        int getArgValue(int pc, int argN, int mode);
        int execute();
};