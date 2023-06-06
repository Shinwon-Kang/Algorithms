#include <iostream>
#include <string>
#include <stack>
#include <deque>

int main()
{
    std::string str, bomb;
    std::cin >> str;
    std::cin >> bomb;
    
    std::deque<char> deq;
    for(auto t : str) {
        deq.push_back(t);
        if(t == bomb.back()) {
            std::stack<char> temp;
            if(deq.size() >= bomb.size()) {
                for(int i=bomb.length()-1; i>=0; i--) {
                    if(deq.empty()) {
                        break;
                    }
                    if(deq.back() == bomb[i]) {
                        temp.push(deq.back());
                        deq.pop_back();
                    } else {
                        break;
                    }
                }
                
                if(temp.size() != bomb.length()) {
                    while(!temp.empty()) {
                        deq.push_back(temp.top());
                        temp.pop();
                    }
                }
            }
        }
    }
    
    if(deq.empty()) {
        std::cout << "FRULA";
    }
    
    for(auto iter = deq.begin(); iter != deq.end(); iter++) {
        std::cout << *iter;
    }
        
    return 0;
}