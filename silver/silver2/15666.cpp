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
    
    int prev_num = -1;
    for(int i=0; i<nums.size(); i++) {
        if((vec.empty()) || (vec.back() <= nums[i])) {
            if(prev_num == nums[i]) {
                continue;
            }
            
            vec.push_back(nums[i]);
            
            dfs(vec);
            
            vec.pop_back();
            
            prev_num = nums[i];
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
    
    std::vector<int> vec;
    dfs(vec);
    
    return 0;
}
