#include <iostream>
#include <stdio.h>

int main()
{
    int C, N;
    scanf("%d %d", &C, &N);
    
    int cost[N], customer[N];
    for(int i=0 ;i<N; i++) {
        int co, cu;
        scanf("%d %d", &co, &cu);
        
        cost[i] = co;
        customer[i] = cu;
    }
    
    int dp[N+1][C+1];
    for(int i=0; i<N+1; i++) dp[i][0] = 1000*101;
    for(int i=0; i<C+1; i++) dp[0][i] = 1000*101;
    
    for(int j=1; j<C+1; j++) {
        for(int i=1; i<N+1; i++) {
            if(customer[i-1] >= j) {
                dp[i][j] = std::min(dp[i-1][j], cost[i-1]);
            } else if(customer[i-1] < j) {
                int x = cost[i-1] + dp[N][j-customer[i-1]];
                int y = dp[i-1][j];
                
                dp[i][j] = std::min(x, y);
                
                if(customer[i-1] > j - customer[i-1]) {
                    dp[i][j] = std::min(dp[i][j], 2*cost[i-1]);
                }
            }
        }
    }
    
    std::cout << dp[N][C];
    
    return 0;
}
