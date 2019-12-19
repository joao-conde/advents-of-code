#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <unordered_map>

using namespace std;

struct MapHash{ 
    size_t operator()(const pair<string,int> &k) const{
        return hash<string>{}(k.first + to_string(k.second));
    }
};

typedef unordered_map<pair<string, int>, vector<pair<string, int>>, MapHash> reactionsList;

unordered_map<string, int> findBasicComponents(string component, const reactionsList &list, unordered_map<string, int> &components){

}

int main(){
    ifstream file("../res/day14");
    string line;
    reactionsList reactions;
    while(getline(file, line)){
        size_t arrow = line.find("=>");
        string recipe = line.substr(0, arrow);
        string result = line.substr(arrow + 3, line.size());    

        int quantity;
        string entry, component;
        vector<pair<string, int>> recipeVec;
        stringstream ss_recipe(recipe);
        while(ss_recipe.good()){
            getline(ss_recipe, entry, ',');
            stringstream ss_entry(entry);
            ss_entry >> quantity >> component;
            recipeVec.push_back(make_pair(component, quantity));
        }

        stringstream ss_result(result);
        ss_result >> quantity >> component;
        pair<string, int> resultPair = make_pair(component, quantity);
        reactions.insert(make_pair(resultPair, recipeVec));
    }

    unordered_map<string, int> fuelBasicComponents;
    pair<string, int> component = make_pair("FUEL", 1);
    findBasicComponents("FUEL", reactions, fuelBasicComponents);
}