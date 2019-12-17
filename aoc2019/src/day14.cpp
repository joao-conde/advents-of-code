#include <fstream>
#include <iostream>
#include <string>

using namespace std;

int main(){
    ifstream file("../res/day14");
    string line;
    while(getline(file, line)){
        cout << line << endl;
    }
}