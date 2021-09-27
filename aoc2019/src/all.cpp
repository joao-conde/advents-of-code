#include <iostream>
#include <string>

using namespace std;

int main(){
    cout << "Advent of Code 2019\n" << endl;
    for(int d = 1; d <= 14; d++) {
        cout << "> Day " << d << endl;
        string day = (d < 10) ? "0" + to_string(d) : to_string(d);
        string command = "g++ src/day" + day + ".cpp -o day";
        system(command.c_str());
        system("./day");
        cout << endl;
    }
}
