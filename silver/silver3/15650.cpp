#include <iostream>
#include <stdio.h>
#include <vector>

void dfs(int N, int M, std::vector<int> vec) {
    if(vec.size() == M) {
        for(int i=0; i<vec.size(); i++) std::cout << vec[i] << " ";
        std::cout << "\n";
        return;
    }
    
    for(int i=vec.back()+1; i<=N-(M-vec.size())+1; i++) {
        vec.push_back(i);
        dfs(N, M, vec);
        vec.pop_back();
    }
}

int main()
{
    int N, M;
    scanf("%d %d", &N, &M);
    
    for(int i=1; i<=N-M+1; i++) {
        std::vector<int> vec {i};
        dfs(N, M, vec);
    }
    
    return 0;
}