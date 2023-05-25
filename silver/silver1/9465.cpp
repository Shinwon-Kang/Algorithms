#include <iostream>
#include <stdio.h>

int main() {
    int N;
    scanf("%d", &N);

    for(int i=0; i<N; i++) {
        int M;
        scanf("%d", &M);

        int board[2][M];
        for(int q=0; q<2; q++){
            for(int j=0; j<M; j++) {
                int temp;
                scanf("%d", &temp);

                board[q][j] = temp;
            }
        }
        
        int dp[2][M];
        dp[0][0] = board[0][0];
        dp[1][0] = board[1][0];
        
        if(M > 1) {
            dp[0][1] = dp[1][0] + board[0][1];
            dp[1][1] = dp[0][0] + board[1][1];
            
            if(M > 2) {
                int step = 2;
                while(step < M) {
                    dp[0][step] = std::max(dp[1][step-1], std::max(dp[0][step-2], dp[1][step-2])) + board[0][step];
                    dp[1][step] = std::max(dp[0][step-1], std::max(dp[0][step-2], dp[1][step-2])) + board[1][step];
                    
                    step++;
                }
            }
        }
        
        std::cout << std::max(dp[0][M-1], dp[1][M-1]) << "\n";

    }

    return 0;
}
