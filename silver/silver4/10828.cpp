#include <iostream>
#include <stdio.h>
#include <string>

class CusTomStack {
public:
    void push(int x) {
        stack_[++pos] = x;
    }
    
    void pop() {
        if(pos == -1) {
            std::cout << -1 << "\n";
            return;
        }
        
        std::cout << stack_[pos] << "\n";
        pos--;
    }
     
    void size() {
        std::cout << (pos+1) << "\n";
    }
    
    void empty() {
        if(pos == -1) {
            std::cout << 1 << "\n";
        } else {
            std::cout << "0" << "\n";
        }
    }
    
    void top() {
        if(pos == -1) {
            std::cout << -1 << "\n";
        } else {
            std::cout << stack_[pos] << "\n";
        }
    } 
    
public:
    int stack_[10000];
    int pos=-1;
};

int main()
{
    int N;
    scanf("%d", &N);
    
    std::string command;
    CusTomStack stk;
    for(int i=0; i<N; i++) {
        std::cin >> command;
        
        if(command == "push") {
            int x;
            std::cin >> x;
            
            stk.push(x);
        } else if(command == "pop") {
            stk.pop();
        } else if(command == "size") {
            stk.size();
        } else if(command == "empty") {
            stk.empty();
        } else if(command == "top") {
            stk.top();
        }
    }

    return 0;
}