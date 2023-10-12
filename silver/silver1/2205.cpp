#include <iostream>
#include <stdio.h>
#include <vector>

int main()
{
    int n;
    scanf("%d", &n);
    
    bool w[10001];
    std::fill_n(w, 10001, false);
    
    std::vector<int> res;
    
    for(int i=n; i>=1; i--) {
        int t = 32768;
        
        while(true) {
            int nee = t - i;
            
            if(nee <= n && w[nee] == false) {
                res.emplace_back(nee);
                w[nee] = true;
                break;
            }
            
            t /= 2;
        }
    }

    for(int i=res.size()-1; i>=0; i--) {
        std::cout << res[i] << "\n";
    }

    return 0;
}
