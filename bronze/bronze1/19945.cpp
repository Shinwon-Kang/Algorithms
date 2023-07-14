#include <iostream>
#include <stdio.h>

using namespace std;

int main()
{
    int n;
    scanf("%d", &n);
    
    if(n == 0) {
        std::cout << "1";
        return 0;
    }
    
    if(n < 0) {
        std::cout << "32";
        return 0;
    }
    
    int cnt = 0;
    int two = 1;
    while(true) {
        if(two > n) {
            break;
        }
        
        two *= 2;
        cnt++;
    }
    
    std::cout << cnt;

    return 0;
}
