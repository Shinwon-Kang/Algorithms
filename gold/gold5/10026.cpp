#include <iostream>
#include <deque>
#include <utility>

using namespace std;

char boxes[100][100];
int visit[100][100];
int ab_visit[100][100];

void rec(int n, int i, int j, int color, int state) {
    if(state == 0 && visit[i][j] != 0) return;
    if(state == 1 && ab_visit[i][j] != 0) { return; }
    
    if(state == 0)
        visit[i][j] = color;
    else {
        ab_visit[i][j] = color;
    }

    if(i-1 >= 0 && boxes[i][j] == boxes[i-1][j]) {
        rec(n, i-1, j, color, state);
    }
    if(i+1 < n && boxes[i][j] == boxes[i+1][j]) {
        rec(n, i+1, j, color, state);
    }
    if(j-1 >= 0 && boxes[i][j] == boxes[i][j-1]) {
        rec(n, i, j-1, color, state);
    }
    if(j+1 < n && boxes[i][j] == boxes[i][j+1]) {
        rec(n, i, j+1, color, state);
    }
    
    if(boxes[i][j] == 'G') boxes[i][j] = 'R';
}

int main()
{
    int N;
    std::cin >>N;
    for(int i=0; i<N; i++) {
        for(int j=0; j<N; j++) {
            std::cin >> boxes[i][j];
            visit[i][j] = 0;
            ab_visit[i][j] = 0;
        }
    }
    
    int area=1;
    for(int i=0; i<N; i++) {
        for(int j=0; j<N; j++) {
            if(visit[i][j] == 0) {
                rec(N, i, j, area++, 0);
            }
        }
    }
    std::cout << area-1 << " ";
    
    area=1;
    for(int i=0; i<N; i++) {
        for(int j=0; j<N; j++) {
            if(ab_visit[i][j] == 0) {
                rec(N, i, j, area++, 1);
            }
        }
    }
    std::cout << area-1;
}
