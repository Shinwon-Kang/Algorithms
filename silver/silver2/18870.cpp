#include <iostream>
#include <vector>
#include <set>
#include <iterator>

using namespace std;

int main()
{
    std::ios::sync_with_stdio(false);
    std::cin.tie(NULL);
    std::cout.tie(NULL);
    
    int N;
    std::cin >> N;
    
    int arr[N];
    set<int> x;
    int temp;
    for(int i=0; i<N; i++) {
        std::cin >> temp;
        arr[i] = temp;
        x.insert(temp);
    }
    
    std::vector<int> v(x.begin(), x.end());
    for (int i=0; i<N; i++) {
        std::cout << std::lower_bound(v.begin(), v.end(), arr[i]) - v.begin() << " ";
    }
    
    return 0;
}
