#include <iostream>
#include <stdio.h>
#include <vector>

#define MAX_NUM 1000001

int main()
{
    std::vector<int> test;
    
    int n;
    int max_test = 0;
    while(true) {
        scanf("%d", &n);
        if(n == 0) break;
        
        test.push_back(n);
        if(max_test < n) max_test = n;
    }
    
    
    bool v[max_test+1];
    std::fill_n(v, max_test+1, true);
    v[0] = false;
    v[1] = false;
    
    std::vector<int> prime;
    
    for(int i=2; i<max_test+1; i++) {
        if(v[i]) {
            int cnt = 2;
            while(i * cnt < max_test+1) {
                v[i * cnt] = false;
                cnt++;
            }
            
            prime.emplace_back(i);
        }
    }
    
    for(int i=0; i<test.size(); i++) {
        for(int j=0; j<prime.size(); j++) {
            int num = test[i] - prime[j];
            if(v[num]) {
                std::cout << test[i] << " = " << prime[j] << " + " << num 
<< "\n";
                break;
            }
        }
    }

    return 0;
}
