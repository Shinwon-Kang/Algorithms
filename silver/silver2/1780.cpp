#include <iostream>
#include <string>
#include <vector>

using namespace std;

std::vector<std::string> map;
std::vector<int> move_co = {0, 1, 2};

std::string func(int x, int y, int s) {
    if(s == 1) {
        if(map[x][y] == '0') return "0";
        else if(map[x][y] == '1') return "1";
        else return "2";
    }
    
    std::string temp = "";
    for(int i=0; i<3; i++) {
        for(int j=0; j<3; j++) {
            temp += func(x+move_co[i]*s/3, y+move_co[j]*s/3, s/3);
        }
    }
    
    if(temp.size() == 9) {
        for(int i=0; i<temp.size()-1; i++) {
            if(temp[i] != temp[i+1]) {
                return temp;
            }
        }
    } else {
        return temp;
    }
    
    if(temp[0] == '0') return "0";
    else if(temp[0] == '1') return "1";
    else return "2";
}

int main()
{
    int N;
    std::cin >> N;
    
    std::string temp;
    for(int i=0; i<N; i++) {
        std::string line = "";
        for(int j=0; j<N; j++) {
            std::cin >> temp;
            if(temp == "-1") temp = "2";
            line += temp;
        }
        map.push_back(line);
    }

    std::string res = func(0, 0, N);
    
    int cnt[3] = {0};
    for(int i=0; i<res.size(); i++) {
        if(res[i] == '2') cnt[0]++;
        else if(res[i] == '0') cnt[1]++;
        else cnt[2]++;
    }
    
    std::cout << cnt[0] << "\n";
    std::cout << cnt[1] << "\n";
    std::cout << cnt[2];
    
    return 0;
}
