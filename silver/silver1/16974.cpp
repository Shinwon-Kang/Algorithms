#include <iostream>
#include <stdio.h>

int main()
{
    long long N, X;
    scanf("%lld %lld", &N, &X);
    
    long long patty[N+1];
    long long burger[N+1];
    patty[0] = 1;
    burger[0] = 1;
    for(int i=1; i<=N; i++) {
        patty[i] = patty[i-1] * 2 + 1;
        burger[i] = burger[i-1] * 2 + 3;
    }
    
    long long eat = 0;
    while(true) {if(N == 0) {
            eat += 1;
        }
        if(X == 1) {
            break;
        }
        if(X == burger[N]/2 +1) {
            eat += (patty[N-1] + 1);
            break;
        }
        if(X == burger[N]) {
            eat += patty[N];
            break;
        }
        
        if(X > burger[N]/2 + 1) {
            eat += (patty[N-1] + 1);
            X = X - (burger[N-1] + 2);
            N--;
        } else {
            N--;
            X -= 1;
        }
    }
    
    std::cout << eat;
    
    
    return 0;
}

