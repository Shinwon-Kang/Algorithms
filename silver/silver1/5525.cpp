#include <iostream>
#include <vector>
#include <string>

using namespace std;

int main()
{
    int n, s;
    std::string input;
    
    std::cin >> n;
    std::cin >> s;
    std::cin >> input;
    
    std::vector<std::string> rul;
    
    char find_ch = 'I';
    std::string m = "";
    for(int i=0; i<input.size(); i++) {
        if(input[i] == find_ch) {
            m += input[i];
            if(find_ch == 'I') find_ch = 'O';
            else find_ch = 'I';
        } else {
            if(input[i] == 'I') {
                rul.push_back(m);
                m = "I";
                find_ch = 'O';
            } else {
            rul.push_back(m.substr(0, m.size()-1));
                m = "";
                find_ch = 'I';
            }
            
            if(rul.back().size() < 3) rul.pop_back();
        }
    }
    if(m.front() == 'I' && m.back() == 'I') rul.push_back(m);
    if(m.front() == 'I' && m.back() == 'O') rul.push_back(m.substr(0, 
m.size()-1));
    
    int cnt=0;
    for(int i=0; i<rul.size(); i++) {
        int t = (rul[i].size() - (n * 2 + 1)) / 2 + 1;
        if(t > 0) cnt+=t;
    }
    
    std::cout << cnt;
    
    return 0;
}
