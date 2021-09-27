#include <fstream>
#include <iostream>
#include <unordered_map>
#include <string>
#include <vector>

using namespace std;

struct Node{
    string id;
    Node* center;
    vector<Node*> moons;
    Node(){}
    Node(string id) : id(id) {}
    void addMoon(Node* moon){ this->moons.push_back(moon); }
    void setCenter(Node* center) { this->center = center; }
};

int nOrbits(Node* node, int depth, unordered_map<string, int> &depths){
    depths.insert(make_pair(node->id, depth));
    int orbits = depth;
    for(Node* moon: node->moons) orbits += nOrbits(moon, depth + 1, depths);
    return orbits;
}

vector<string> pathFromRoot(Node* n, Node* root){
    vector<string> path = { n->id };
    while(n->id != root->id){
        path.insert(path.begin(), n->center->id);
        n = n->center;
    }
    return path;
}

string deepestCommonAncestor(vector<string> s1, vector<string> s2){
    string deepestCommonAncestor;
    vector<string> big = (s1.size() >= s2.size() ? s1 : s2);
    vector<string> small = (s1.size() < s2.size() ? s1 : s2);
    for(int i = 0; i < small.size(); i++){
        if(small[i] == big[i]) deepestCommonAncestor = small[i];
        else break;
    }
    return deepestCommonAncestor;
}

int main(){
    string line;
    ifstream input("input/day06");
    unordered_map<string, Node> nodes;
    unordered_map<string, int> depths;
    while(getline(input, line)){
        int separatorPos = line.find(")");
        string centerID = line.substr(0, separatorPos), moonID = line.substr(separatorPos + 1, line.size());

        auto itCenter = nodes.find(centerID);
        if(itCenter == nodes.end()){
            Node center(centerID);
            nodes.insert(make_pair(centerID, center));
            itCenter = nodes.find(centerID);
        }

        auto itMoon = nodes.find(moonID);
        if(itMoon == nodes.end()){
            Node moon(moonID);
            nodes.insert(make_pair(moonID, moon));
            itMoon = nodes.find(moonID);
        }

        itCenter->second.addMoon(&itMoon->second);
        itMoon->second.setCenter(&itCenter->second);
    }
    input.close();

    cout << "Part1: " << nOrbits(&nodes["COM"], 0, depths) << " orbits." << endl;

    vector<string> youPath = pathFromRoot(&nodes["YOU"], &nodes["COM"]);
    vector<string> sanPath = pathFromRoot(&nodes["SAN"], &nodes["COM"]);
    string deepestAncestor = deepestCommonAncestor(youPath, sanPath);
    int youParentDepth = depths[nodes["YOU"].center->id], sanParentDepth = depths[nodes["SAN"].center->id];
    int orbitalTransfers = (youParentDepth - depths[deepestAncestor]) + (sanParentDepth - depths[deepestAncestor]);
    cout << "Part2: " << orbitalTransfers << " minimum orbital transfers required." << endl;
}
