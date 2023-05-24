#include <iostream>
#include <stdio.h>
#include <queue>

int main() {
    int N;
    scanf("%d", &N);

    std::priority_queue<int, std::vector<int>, std::greater<int>> q;

    for(int i=0; i<N; i++) {
        int temp;
        scanf("%d", &temp);

        q.push(temp);
    }

    int total = 0;
    int prev = 0;
    while(!q.empty()) {
        total = total + (prev + q.top());
        prev = prev + q.top();
        
        q.pop();
    }

    std::cout << total << "\n";

    return 0;
}
