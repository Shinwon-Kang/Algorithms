#include <iostream>
#include <stdio.h>
#include <vector>
#include <utility>
#include <algorithm>

int n;
int max_r = 0;
std::vector<std::pair<int, int>> vec[10001];

int dfs(int parent) {
    if(vec[parent].empty()) {
        return 0;
    }
    
    std::vector<std::pair<int, int>>& childs = vec[parent];
    std::vector<int> weights;
    int max_weight = 0;
    for(int i=0; i<childs.size(); i++) {
        int child = childs[i].first;
        int weight = childs[i].second;

        int r = dfs(child);
        if(r + weight > max_weight) {
            max_weight = r + weight;
        }

        weights.push_back(r + weight);
    }
    sort(weights.begin(), weights.end(), std::greater<int>());

    int r = weights[0];
    if(weights.size() > 1) {
        r += weights[1];
    }

    if(max_r < r) {
        max_r = r;
    }

    return max_weight;
};

int main()
{
    scanf("%d", &n);
    
    int a, b, c;
    for(int i=0; i<n-1; i++) {
        scanf("%d %d %d", &a, &b, &c);
        vec[a].push_back(std::make_pair(b, c));
    }
    
    int max = dfs(1);

    std::cout << max_r;

    return 0;
}
