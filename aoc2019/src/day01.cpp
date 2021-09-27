#include <algorithm>
#include <fstream>
#include <iostream>
#include <iterator>
#include <numeric>
#include <string>
#include <vector>

using namespace std;

int calculateTotalFuel(vector<int> &mass){
    transform(mass.begin(), mass.end(), mass.begin(), [](int m){ return m/3 - 2;});
    mass.erase(remove_if(mass.begin(), mass.end(), [](int m){return m <= 0;}), mass.end()); //for part2 recursion
    return accumulate(mass.begin(), mass.end(), 0, [](int m1, int m2){return m1 + m2;});
}

int main(){
    // read mass file to vector
    vector<int> mass;
    ifstream input("input/day01");
    istream_iterator<int> iit(input), eos;
    copy(iit, eos, back_inserter(mass));
    input.close();

    // Part1
    // map -> reduce
    int totalFuel = calculateTotalFuel(mass);
    cout << "Part1: " << totalFuel << endl;

    // Part2
    // map -> reduce -> filter -> map -> ... -> reduce
    while(!mass.empty()) totalFuel += calculateTotalFuel(mass);
    cout << "Part2: " << totalFuel << endl;
}
