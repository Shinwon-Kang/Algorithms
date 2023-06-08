#include <iostream>
#include <stdio.h>

int main()
{
    int N;
    scanf("%d", &N);
    
    int min_max_board[3][2];
    for(int i=0; i<N; i++) {
        int x, y, z;
        scanf("%d %d %d", &x, &y, &z);
        
        int min_, max_;
        if(i == 0) {
            min_max_board[0][0] = x;
            min_max_board[0][1] = x;
            min_max_board[1][0] = y;
            min_max_board[1][1] = y;
            min_max_board[2][0] = z;
            min_max_board[2][1] = z;
        } else {
            int min0 = x + std::min(min_max_board[0][0], 
min_max_board[1][0]);
            int max0 = x + std::max(min_max_board[0][1], 
min_max_board[1][1]);
             
            int min1 = y + std::min(std::min(min_max_board[0][0], 
min_max_board[1][0]), min_max_board[2][0]);
            int max1 = y + std::max(std::max(min_max_board[0][1], 
min_max_board[1][1]), min_max_board[2][1]);
            
            int min2 = z + std::min(min_max_board[1][0], 
min_max_board[2][0]);
            int max2 = z + std::max(min_max_board[1][1], 
min_max_board[2][1]);
            
            min_max_board[0][0] = min0;
            min_max_board[0][1] = max0;

            min_max_board[1][0] = min1;
            min_max_board[1][1] = max1;

            min_max_board[2][0] = min2;
            min_max_board[2][1] = max2;
        }
    }
    
    int min_v = std::min(std::min(min_max_board[0][0], 
min_max_board[1][0]), min_max_board[2][0]);
    int max_v = std::max(std::max(min_max_board[0][1], 
min_max_board[1][1]), min_max_board[2][1]);
    
    std::cout << max_v << " " << min_v;

    return 0;
}
