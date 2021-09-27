#include <iostream>
#include <set>
#include <sstream>

using namespace std;

string encode(int password) {
    int prefixCnt = 0;
    string encodedPW = "";
    stringstream ss(to_string(password));
    char prefix = ss.peek();
    while (!ss.eof()) {
        if (prefix != ss.peek()) {
            encodedPW += (prefix + to_string(prefixCnt));
            prefix = ss.peek();
            prefixCnt = 1;
        } else {
            prefixCnt++;
        }

        ss.ignore();
    }
    return encodedPW;
}

bool isValidPassword(int password, bool exactReplication) {
    bool doubleFound = false;
    stringstream ss(encode(password));
    while (!ss.eof()) {
        char letter = ss.get(), mult = ss.get(), nextLetter = ss.peek();
        if (exactReplication) {
            // exact double rule
            if (atoi(&mult) == 2) doubleFound = true;
        } else {
            // "lazy" double rule
            if (atoi(&mult) >= 2) doubleFound = true;
        }
        // ever increasing order rule
        if (letter > nextLetter && nextLetter != EOF) return false;
    }
    return doubleFound;
}

int main() {
    int uniquePwsP1 = 0, uniquePwsP2 = 0, lb = 278384, ub = 824795;
    for (int i = lb; i <= ub; i++) {
        if (isValidPassword(i, false)) uniquePwsP1++;
        if (isValidPassword(i, true)) uniquePwsP2++;
    }
    cout << "Part1 - " << uniquePwsP1 << endl;
    cout << "Part2 - " << uniquePwsP2 << endl;
}
