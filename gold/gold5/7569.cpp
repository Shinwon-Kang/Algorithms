#include <iostream>
#include <deque>
#include <utility>

using namespace std;

int main()
{
    int M, N, H;
    std::cin >> M >> N >> H;
    int boxes[N*H][M];
    for(int i=0; i<N*H; i++) {
        for(int j=0; j<M; j++) {
            std::cin >> boxes[i][j];
        }
    }
    
    std::deque<std::pair<int, int>> will_visit;
    int day = 0;
    int no_to = 0;
    
    for(int i=0; i<N*H; i++) {
        for(int j=0; j<M; j++) {
            if(boxes[i][j] == 0) {
                no_to++;
            }
            if(boxes[i][j] == 1) {
                will_visit.push_back(std::pair<int, int>(i, j));
            }
        }
    }
    
    while(will_visit.size() != 0) {
        day++;
        int times_one_day = will_visit.size();
        for(int d=0; d<times_one_day; d++){
            std::pair<int, int> coord = will_visit.front();
            will_visit.pop_front();
            
            int stair = (coord.first) / N;
            
            if(((coord.second - 1) >= 0) && (boxes[coord.first][coord.second - 1] == 0)) {
                boxes[coord.first][coord.second - 1] = 1;
                will_visit.push_back(std::pair<int, int>(coord.first, coord.second - 1));
                no_to--;
            }
            if(((coord.second + 1) < M) && (boxes[coord.first][coord.second + 1] == 0)) {
                boxes[coord.first][coord.second + 1] = 1;
                will_visit.push_back(std::pair<int, int>(coord.first, coord.second + 1));
                no_to--;
            }
            if(((coord.first - 1) >= (stair-1)*N + N) && (boxes[coord.first - 1][coord.second] == 0)) {
                boxes[coord.first - 1][coord.second] = 1;
                will_visit.push_back(std::pair<int, int>(coord.first - 1, coord.second));
                no_to--;
            }
            if(((coord.first + 1) < (stair+1)*N) && (boxes[coord.first + 1][coord.second] == 0)) {
                boxes[coord.first + 1][coord.second] = 1;
                will_visit.push_back(std::pair<int, int>(coord.first + 1, coord.second));
                no_to--;
            }
            if(((coord.first + N) < N*H) && (boxes[coord.first + N][coord.second] == 0)) {
                boxes[coord.first + N][coord.second] = 1;
                will_visit.push_back(std::pair<int, int>(coord.first + N, coord.second));
                no_to--;
            }
            if(((coord.first - N) >= 0) && (boxes[coord.first - N][coord.second] == 0)) {
                boxes[coord.first - N][coord.second] = 1;
                will_visit.push_back(std::pair<int, int>(coord.first - N, coord.second));
                no_to--;
            }
        }
    }
    
    if(no_to != 0) std::cout << -1 << "\n";
    else std::cout << day-1 << "\n";
}
