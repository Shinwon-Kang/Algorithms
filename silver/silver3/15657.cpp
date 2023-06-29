#include <iostream>
#include <iostream>
#include <vector>
#include <algorithm>

int N, M;
std::vector<int> nums;

void dfs(std::vector<int> vec) {
    if(vec.size() == M) {
        for(int i=0; i<M; i++) {
            std::cout << vec[i] << " ";
        }
        std::cout << "\n";

        return;
    }
    
    for(int i=0; i<nums.size(); i++) {
        if(vec.back() <= nums[i]) {
            vec.push_back(nums[i]);
            dfs(vec);
            vec.pop_back();            
        }
    }
}

int main()
{
    scanf("%d %d", &N, &M);
    
    for(int i=0; i<N; i++) {
        int temp;
        scanf("%d", &temp);
        
        nums.push_back(temp);
    }
    sort(nums.begin(), nums.end());
    
    if(M == 1) {
        for(int i=0; i<N; i++) {
            std::cout << nums[i] << "\n";
        }
        return 0;
    }
    
    for(int i=0; i<N; i++) {
        std::vector<int> vec;
        vec.push_back(nums[i]);
        dfs(vec);        
    }
    
    return 0;
}
