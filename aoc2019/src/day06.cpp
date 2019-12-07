#include <fstream>
#include <iostream>
#include <unordered_map>
#include <string>
#include <vector>

using namespace std;

struct Node{
    string id, center; //center is the 'parent' on which this node rotates
    vector<string> moons;
    Node(){}
    Node(string id) : id(id) {}
    void addMoon(string id){ moons.push_back(id); }
    void setCenter(string id) { center = id; }
};

int nOrbits(Node node, int depth, unordered_map<string, Node> &nodes, unordered_map<string, int> &depths){
    cout << "node " << node.id << " at depth " << depth << endl;
    depths.insert(make_pair(node.id, depth));
    int orbits = depth;
    for(string moonID: node.moons) orbits += nOrbits(nodes[moonID], depth + 1, nodes, depths);
    return orbits;
}

vector<string> pathFromRoot(Node n, Node root, unordered_map<string, Node> &nodes){
    vector<string> path = { n.id };
    while(n.id != root.id){
        path.insert(path.begin(), n.center); 
        n = nodes[n.center];
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
    ifstream input("../res/day06"); 
    unordered_map<string, Node> nodes;
    unordered_map<string, int> depths;
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
        nodes[moon].setCenter(center);
    }
    input.close();

    cout << "Part1: " << nOrbits(nodes["COM"], 0, nodes, depths) << endl;
    for(auto kv: depths) cout << "node " << kv.first << " at depth " << kv.second << endl;
    // cout << depths["D"] << endl;
    // vector<string> youPath = pathFromRoot(nodes["YOU"], nodes["COM"], nodes);
    // vector<string> sanPath = pathFromRoot(nodes["SAN"], nodes["COM"], nodes);
    // string ancestor = deepestCommonAncestor(youPath, sanPath);

    // for(string c: youPath) cout << c << " ";
    // cout << endl;
    // for(string c: sanPath) cout << c << " ";
    // cout << endl;
    // for(string c: lcp) cout << c << " ";
    // cout << endl;

    // cout << ancestor << endl;
    // cout << nOrbits(nodes["D"], 0, nodes) << endl;
    // cout << "Part2: " <<  (youPath.size() - lcp.size()) + (sanPath.size() - lcp.size()) << endl;
}