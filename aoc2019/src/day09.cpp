#include <fstream>
#include <iostream>
#include <vector>

#include "Intcode.cpp"

using namespace std;

int main(){
    vector<long long int> intcode;
    ifstream input("input/day09");
    while(!input.eof()){
        long long int code;
        input >> code;
        input.ignore();
        intcode.push_back(code);
    }
    input.close();

    IntcodeProgram program1(intcode, intcode.size()), program2(intcode, intcode.size());
    cout << "Part1: BOOST keycode " << program1.execute({1}) << endl;
    cout << "Part2: Distress signal coordinates " << program2.execute({2}) << endl;
}
