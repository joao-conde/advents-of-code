#include <fstream>
#include <iostream>
#include <vector>

using namespace std;

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

    IntcodeProgram program(intcode, 2 * intcode.size());
    cout << program.execute({1}) << endl;
}