#include <iostream>
#include <vector>

using namespace std;
std::vector<std::string> map;

std::string func(int x, int y, int s) {
    if(s == 1) {
        if(map[x][y] == '0') return "0";
        else return "1";
    }
    
    std::vector<std::string> res;
    
    res.push_back(func(x, y, s/2));
    res.push_back(func(x, y+s/2, s/2));
    res.push_back(func(x+s/2, y, s/2));
    res.push_back(func(x+s/2, y+s/2, s/2));
    
    std::string re_str = "";
    int go=0;
    for(int i=0; i<res.size(); i++) {
        if(res[i].size() == 1) {
            re_str += res[i];
        } else {
            re_str += "(" + res[i] + ")";
            go=1;
        }
    }
    
    if(re_str.size() == 4) {
        for(int i=0; i<re_str.size()-1; i++) {
            if(re_str[i] != re_str[i+1]) {
                return re_str;
            }
        }
    } else {
        return re_str;
    }
    
    if(re_str[0] == '0') return "0";
    else return "1";
}

int main()
{
    int N;
    std::cin >> N;
    
    std::string temp;
    for(int i=0; i<N; i++) {
        std::cin >> temp;
        map.push_back(temp);
    }
    
    std::string res = func(0, 0, N);
    if(res.size() == 1) {
        std::cout << res;
    } else{
        std::cout << "(" << res << ")";
    }

    return 0;
}
