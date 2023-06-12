#include <iostream>
#include <stdio.h>
#include <vector>

int main()
{
    int N;
    scanf("%d", &N);
    
    std::vector<int> hexa {1};
    int idx = 2;
    while(true) {
        int h = hexa[idx-2] + ((6 * (idx-1)) - (2 * (idx-1) - 1));
        
        if(h == N) {
            std::cout << 1;
            return 0;
        } else if (h < N) {
            hexa.emplace_back(h);
        } else {
            break;
        }
        idx++;
    }
    
    int dp[1000001] = {0, 1, 2, 3, 4, 5, 1, 2, 3, 4, 5, 6, 2};
    if(N>=1 && N<=12) {
        std::cout << dp[N];
        return 0;
    }
    
    for(int i=13; i<N+1; i++) {
        int min = 6;
        for(int j=0; j<hexa.size(); j++) {
            if(hexa[j] == i) {
                min = 1;
                break;
            } else if(hexa[j] < i) {
                min = std::min(min, dp[i - hexa[j]]+1);
            } else {
                break;
            }
        }
        dp[i] = min;
    }
    
    std::cout << dp[N];
    
    return 0;
}