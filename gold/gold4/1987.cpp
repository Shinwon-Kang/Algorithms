#include <iostream>
#include <stdio.h>
#include <string>
#include <set>

int R, C;
int max_path = 0;

void dfs(bool* visited, char** board, int x, int y, int cnt) {
    bool flag = false;
    
    if(x - 1 >= 0) {
        if(!visited[board[x-1][y] - 65]) {
            visited[board[x-1][y] - 65] = true;
            dfs(visited, board, x-1, y, cnt+1);
            visited[board[x-1][y] - 65] = false;
        }
    }
    
    if(x + 1 < R) {
        if(!visited[board[x+1][y] - 65]) {
            visited[board[x+1][y] - 65] = true;
            dfs(visited, board, x+1, y, cnt+1);
            visited[board[x+1][y] - 65] = false;
        }
    }
    
    if(y - 1 >= 0) {
        if(!visited[board[x][y-1] - 65]) {
            visited[board[x][y-1] - 65] = true;
            dfs(visited, board, x, y-1, cnt+1);
            visited[board[x][y-1] - 65] = false;
        }
    }
    
    if(y + 1 < C) {
        if(!visited[board[x][y+1] - 65]) {
            visited[board[x][y+1] - 65] = true;
            dfs(visited, board, x, y+1, cnt+1);
            visited[board[x][y+1] - 65] = false;
        }  
    }
    
    if(!flag) {
        if(max_path < cnt) {
            max_path = cnt;
        }
    }
}

int main()
{
    scanf("%d %d", &R, &C);
    
    char **board;
    board = new char*[R];
    for(int i=0; i<R; i++) {
        std::string temp;
        std::cin >> temp;
        board[i] = new char[C];
        for(int j=0; j<C; j++) {
            board[i][j] = temp[j];
        }
    }

    bool visited[26] = {false};
    visited[board[0][0]-65] = true;
    dfs(visited, board, 0, 0, 1);
    
    std::cout << max_path;
    return 0;
}
