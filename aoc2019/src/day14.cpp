#include <cmath>
#include <fstream>
#include <iostream>
#include <sstream>
#include <string>
#include <vector>
#include <unordered_map>

using namespace std;

typedef unordered_map<string, pair<unordered_map<string, int>, int>> reactionsList;

bool isBasicComponent(string component, const reactionsList &reactions) {
    unordered_map<string, int> inputs = reactions.find(component)->second.first;
    auto oreIT = inputs.find("ORE");
    return (oreIT != inputs.end() && inputs.size() == 1);
}

void findBasicComponentsQuantity(string component, int mult, unordered_map<string, int> &list, const reactionsList &reactions) {
    unordered_map<string, int> inputs = reactions.find(component)->second.first;
    for (auto kv: inputs) {
        if (isBasicComponent(kv.first, reactions))
            list[kv.first] += kv.second * mult;
        else
            findBasicComponentsQuantity(kv.first, kv.second * mult, list, reactions);
    }
}

int computeMinimumORE(const unordered_map<string, int> &inputs, const reactionsList &reactions) {
    int ore = 0;

    for (auto kv: inputs) {
        pair<unordered_map<string, int>, int> basicInputs = reactions.find(kv.first)->second;
        int reactCnt = ceil(static_cast<double>(kv.second) / basicInputs.second);

        int tmp = reactCnt * basicInputs.first.find("ORE")->second;
        ore += tmp;

        cout << kv.first << endl;
        cout << "Produced - ORE: " << basicInputs.second * reactCnt << " - " << tmp << endl;
    }

    return ore;
}

int main() {
    // TODO(@joao-conde): refactor to a tree structure and walk it since FUEL and continuously update ore needed
    ifstream file("input/day14");
    string line;
    reactionsList reactions;
    while (getline(file, line)) {
        size_t arrow = line.find("=>");
        string recipe = line.substr(0, arrow);
        string result = line.substr(arrow + 3, line.size());

        int quantity;
        string entry, component;
        unordered_map<string, int> recipeQuantities;
        stringstream ss_recipe(recipe);
        while (ss_recipe.good()) {
            getline(ss_recipe, entry, ',');
            stringstream ss_entry(entry);
            ss_entry >> quantity >> component;
            recipeQuantities.insert(make_pair(component, quantity));
        }

        stringstream ss_result(result);
        ss_result >> quantity >> component;
        reactions.insert(make_pair(component, make_pair(recipeQuantities, quantity)));
    }

    // for(auto kv: reactions){
    //     cout << kv.second.second << " units of component " << kv.first << " can be done with:" << endl;
    //     for(auto r: kv.second.first){
    //         cout << "- " << r.second << " x " << r.first << endl;
    //     }
    // }

    unordered_map<string, int> fuelBasicComponents;
    findBasicComponentsQuantity("FUEL", 1, fuelBasicComponents, reactions);

    for (auto kv: fuelBasicComponents) {
        cout << "Need " << kv.second << " x " << kv.first << endl;
    }

    cout << computeMinimumORE(fuelBasicComponents, reactions) << endl;
}
