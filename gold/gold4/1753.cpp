#include <iostream>
#include <stdio.h>
#include <vector>
#include <utility>
#include <algorithm>
#include <queue>

#define INF 100000000

struct compare {
    bool operator()(std::pair<int, int> a, std::pair<int, int> b) {
        return a.second > b.second;
    }
};

int main() {
    int V, E;
    scanf("%d %d", &V, &E);

    int start_node;
    scanf("%d", &start_node);

    std::vector<std::pair<int, int>> graph[V+1];

    int dist[V+1];
    std::fill_n(dist, V+1, INF);
    dist[start_node] = 0;

    std::priority_queue<std::pair<int, int>, std::vector<std::pair<int, 
int>>, compare> pq;

    int a, b, c;
    for(int i=0; i<E; i++) {
        scanf("%d %d %d", &a, &b, &c);
        graph[a].push_back(std::make_pair(b, c));

        if(a == start_node && dist[b] > c) {
            pq.push(std::make_pair(b, c));
            dist[b] = c;
        }
    }

    while(!pq.empty()) {
        int min_edge = pq.top().first;
        int min_dist = pq.top().second;

        pq.pop();

        if(dist[min_edge] < min_dist) {
            continue;
        }
        
        for(auto edge : graph[min_edge]) {
            int next_node = edge.first;
            int next_dist = edge.second;

            if(dist[min_edge] + next_dist < dist[next_node]) {
                dist[next_node] = dist[min_edge] + next_dist;
                pq.push(std::make_pair(next_node, dist[next_node]));
            }
        }
    }
    
    for(int i=1; i<=V; i++) {
        if(dist[i] == INF) {
            std::cout << "INF\n";
        } else {
            std::cout << dist[i] << "\n";
        }
    }
    return 0;
}
