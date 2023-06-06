#include <iostream>
#include <stdio.h>
#include <string>

int main()
{
    char word1[1000], word2[1000];
    scanf("%s", word1);
    scanf("%s", word2);
    
    std::string w1(word1);
    std::string w2(word2);
    
    int score[w1.length()+1][w2.length()+1];
    for(int i=0; i<w1.length()+1; i++) score[i][0] = 0;
    for(int i=0; i<w2.length()+1; i++) score[0][i] = 0;

    for(int i=1; i<w1.length()+1; i++) {
        for(int j=1; j<w2.length()+1; j++) {
            if(w1[i-1] == w2[j-1]) {
                score[i][j] = score[i-1][j-1] + 1;
            }
            else {
                score[i][j] = std::max(score[i-1][j], score[i][j-1]);
            }
        }
    }
   
    std::cout << score[w1.length()][w2.length()] << "\n";
    

    return 0;
}