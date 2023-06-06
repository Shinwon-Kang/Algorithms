#include <iostream>
#include <string>

int main() {
    std::string L, R;
    std::cin >> L;
    std::cin >> R;
    
    int cnt = 0;
    if(L.length() != R.length()) {
        std::cout << 0;
        return 0;
    }
    
    for(int i=1; i<=L.length(); i++) {
        if((L[L.length() - i] == R[R.length() - i])) {
            if(L[L.length() - i] == '8') {
                cnt++;
            }
        } else {
            cnt = 0;
        }
    }
    
    std::cout << cnt;
    

    return 0;
}