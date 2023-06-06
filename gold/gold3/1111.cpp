#include <iostream>
#include <stdio.h>
#include <vector>


int main()
{
    int N;
    scanf("%d", &N);
    
    std::vector<int> numbers;
    for(int i=0; i<N; i++) {
        int temp;
        std::cin >> temp;
        numbers.push_back(temp);
    }
    
    if(N == 1) {
        std::cout << "A";
        return 0;
    } else if (N == 2) {
        if(numbers[0] == numbers[1]) {
            std::cout << numbers[0];
        } else {
            std::cout << "A";
        }
        return 0;
    }
    
    std::vector<std::pair<int, int>> ans;
    for(int i=1; i<N-1; i++) {
        int x = numbers[i] - numbers[i+1];
        int y = numbers[i-1] - numbers[i];
        
        int a, b;
        if(y == 0) {
            a = 0;
            b = numbers[i];
            
            if(numbers[i] != numbers[i+1]) {
                std::cout << "B";
                return 0;
            }
            
        } else if (x % y == 0) {
            a = x / y;
            b = numbers[i] - (numbers[i-1] * a);
        } else {
            std::cout << "B";
            return 0;
        }

        if(!ans.empty()) {
            if(a != ans.back().first || b != ans.back().second) {
                std::cout << "B";
                return 0;
            }
        }
        ans.emplace_back(a, b);
    }
    
    int v = numbers[N-1] * ans.back().first + ans.back().second;
    std::cout << v;
    
    return 0;
}