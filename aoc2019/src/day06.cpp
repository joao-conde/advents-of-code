#include <fstream>
#include <iostream>
#include <unordered_map>
#include <vector>

using namespace std;

struct Node{
    string id;
    vector<string> moons;
    Node(){}
    Node(string id) : id(id){}
    void addMoon(string id){ moons.push_back(id); }
};

int nOrbits(Node node, int depth, unordered_map<string, Node> &nodes){
    auto it = nodes.find(node.id); //'leaf' moon case
    if(it == nodes.end()) return depth;

    int orbits = depth;
    for(string moonID: node.moons) orbits += nOrbits(nodes[moonID], depth + 1, nodes);
    
    return orbits;
}

int main(){
    string line;
    ifstream input("../res/day06");
    unordered_map<string, Node> nodes;
    while(getline(input, line)){
        int separatorPos = line.find(")");
        string center = line.substr(0, separatorPos), moon = line.substr(separatorPos + 1, line.size());
        
        auto it = nodes.find(center);
        if(it != nodes.end()) it->second.addMoon(moon);
        else {
            Node n(center); 
            n.addMoon(moon);
            nodes.insert(make_pair(center, n));
        }        
    }
    input.close();

    Node root = nodes["COM"];
    cout << nOrbits(root, 0, nodes) << endl;
}