#include <iostream>
#include <stdio.h>

int main()
{
    int N, M;
    scanf("%d %d", &N, &M);
    
    int sum=0;
    int num=1;
    int cnt=1;
    for(int i=1; i<=M; i++) {
        if(i >= N) {
            sum+=num;
        }
        
        if(cnt % num == 0) {
            num++;
            cnt=1;
        } else {
            cnt++;
        }
    }
    
    std::cout << sum;

    return 0;
}