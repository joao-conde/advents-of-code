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
        unordered_map<int, int> opcodePCOffset;
        unordered_map<int, function<int (int, int)>> opcodeBinFun;

    public:
        queue<int> inputs;
        IntcodeProgram(vector<int> intcode);
        void setHalt();
        void offsetPC(int offset);
        void jump(int pos);
        void processBinOpcode(int x, int y, int pos, function<int (int, int)> binFun, int offset);
        void consumeInput(int pos);
        void addInputs(vector<int> inputs);
        bool halted();
        vector<int> getParameterModes(int opcode);
        int get(int position);
        int getArgValue(int pc, int argN, int mode);
        int execute();
        int execute(vector<int> inputs);
};