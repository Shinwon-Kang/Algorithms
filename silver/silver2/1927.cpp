#include <iostream>
#include <queue>
#include <vector>
#include <stdio.h>

int main()
{
    int N;
    std::cin >> N;
    
    std::priority_queue<int, std::vector<int>, std::greater<int>> q;
    
    int temp;
    for(int i=0; i<N; i++) {
        scanf("%d", &temp);
        
        if(temp == 0) {
            if(q.empty()) {
                std::cout << 0 << "n";
            } else {
                std::cout << q.top() << "n";
                q.pop();
            }
        } else {
            q.push(temp);   
        }
    }

    return 0;
}

// input:
// 5
// 1 2 3 4 5
// 1+3+6+9+13 = 32
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
    for(int i=0; i<q.size(); i++) {
        total = total + (prev + q.top());
        prev = prev + q.top();
    }

    std::cout << total << "\n";

    return 0;
}