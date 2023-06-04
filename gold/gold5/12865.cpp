#include <iostream>
#include <stdio.h>

using namespace std;

int main()
{
    int N, M;
    scanf("%d %d", &N, &M);
    int score[N+1][M+1] = {0,};
    
    int bags[N][2];
    for(int i=0; i<N; i++) {
        int w, v;
        scanf("%d %d", &w, &v);
        
        bags[i][0] = w;
        bags[i][1] = v;
        
        score[i+1][w] = v;
    }

    
    for(int j=0; j<M+1; j++) {
        for(int i=0; i<N+1; i++) {
            if(i == 0 || j == 0) {
                score[i][j] = 0;
            } else {
                if(bags[i-1][0] <= j) {
                    score[i][j] = std::max(score[i-1][j], bags[i-1][1] + score[i-1][j - bags[i-1][0]]);
                } else {
                    score[i][j] = score[i-1][j];
                }
            }
        }
    }
    std::cout << score[N][M] << "\n";

    return 0;
}
