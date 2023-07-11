#include <iostream>
#include <stdio.h>

int main() {
    int x, y;
    scanf("%d %d", &x, &y);
    
    int cnt = 1;
    while(x < y) {
        if(y % 2 == 0) {
            y = y / 2;
        } else {
            if(y % 10 == 1) {
                y = y / 10;
            } else {
                std::cout << "-1";
                return 0;
            }
        }
        
        cnt++;
    }
    
    if(x == y) 
        std::cout << cnt;
    else
        std::cout << "-1";
        
    return 0;
}
