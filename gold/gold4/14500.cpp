#include <iostream>
#include <vector>

using namespace std;

int N, M;
int max_value=0;
int visit[500][500] = {0};
int values[500][500] = {0};

void cc(int i, int j) {
    int score;
    if(j-1 >= 0 && j+1 < M && i-1 >= 0) {
        score = values[i][j] + values[i][j-1] + values[i][j+1] + values[i-1][j];
        if(score > max_value) {
            max_value = score;
        }
    }
    
    if(j-1 >= 0 && j+1 < M && i+1 < N) {
        score = values[i][j] + values[i][j-1] + values[i][j+1] + values[i+1][j];
        if(score > max_value) {
            max_value = score;
        }
    }
    
    if(i-1 >= 0 && i+1 < N && j-1 >= 0) {
        score = values[i][j] + values[i-1][j] + values[i+1][j] + values[i][j-1];
        if(score > max_value) {
            max_value = score;
        }
    }
    
    if(i-1 >= 0 && i+1 < N && j+1 < M) {
        score = values[i][j] + values[i-1][j] + values[i+1][j] + values[i][j+1];
        if(score > max_value) {
            max_value = score;
        }
    }
}

void rec(int cnt, int score, int i, int j) {
    if(cnt == 4) {
        if(score > max_value) {
            max_value = score;
        }
        return;
    }
    
    if(j-1 >= 0 && visit[i][j-1] == 0) {
        visit[i][j-1] = 1;
        rec(cnt+1, score+values[i][j-1], i, j-1);
        visit[i][j-1] = 0;
    }
    if(j+1 < M && visit[i][j+1] == 0) {
        visit[i][j+1] = 1;
        rec(cnt+1, score+values[i][j+1], i, j+1);
        visit[i][j+1] = 0;                    
    }
    if(i-1 >= 0 && visit[i-1][j] == 0) {
        visit[i-1][j] = 1;
        rec(cnt+1, score+values[i-1][j], i-1, j);
        visit[i-1][j] = 0;                       
    }
    if(i+1 < N && visit[i+1][j] == 0) {
        visit[i+1][j] = 1;
        rec(cnt+1, score+values[i+1][j], i+1, j);
        visit[i+1][j] = 0;                              
    }
}


int main()
{
    cin >> N >> M;
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            int t;
            cin >> t;
            values[i][j] = t;
        }
    }
    
    
    for (int i = 0; i < N; i++) {
        for (int j = 0; j < M; j++) {
            visit[i][j] = 1;
            rec(1, values[i][j], i, j);
            cc(i, j);
            visit[i][j] = 0;
        }
    }
    
    cout << max_value << "\n";
    
    return 0;
}
