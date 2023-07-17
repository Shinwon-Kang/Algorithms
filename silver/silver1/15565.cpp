#include <stdio.h>
#include <iostream>
#include <vector>

int main() {
    int N, K;
    scanf("%d %d", &N, &K);

    std::vector<int> nums;
    std::vector<int> ptr;
    int num;
    for(int i=0; i<N; i++) {
        scanf("%d", &num);
        if(num == 1) {
            ptr.emplace_back(i);
        }
        nums.emplace_back(num);
    }

    if(ptr.size() < K) {
        std::cout << "-1";
        return 0;
    }

    if(ptr.size() == K) {
        std::cout << (ptr.back() - ptr.front() + 1);
        return 0;
    }

    int min_set = N;
    for(int i=0; i<ptr.size()-(K-1); i++) {
        if((ptr[i+(K-1)] - ptr[i] + 1) < min_set) {
            min_set = ptr[i+(K-1)] - ptr[i] + 1;
        }
    }
    
    std::cout << min_set;

    return 0;
}
