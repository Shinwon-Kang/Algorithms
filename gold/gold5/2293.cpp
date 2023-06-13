#include <iostream>
#include <stdio.h>
#include <vector>

int main()
{
    int n, k;
    scanf("%d %d", &n, &k);
    
    int coins[n];
    for(int i=0; i<n; i++) {
        std::cin >> coins[i];
    }
    
    int dp[k+1] = {1};
    for(int i=0; i<n; i++) {
        for(int j=1; j<k+1; j++) {
            if(coins[i] <= j) {
                int r = j - coins[i];
                dp[j] = dp[r] + dp[j];
            }
        }
    }
    
    std::cout << dp[k];
    
    return 0;
}
