#include <iostream>
#include <vector>

#define DIV 1000000007

std::vector<long long> fib(long long n) {
    if(n == 1) {
        return std::vector<long long> {1, 1, 1, 0};
    }
    
    if(n % 2 == 0) {
        std::vector<long long> t = fib(n/2);
        return std::vector<long long> {
            (t[0] * t[0] + t[1] * t[2]) % DIV,
            (t[0] * t[1] + t[1] * t[3]) % DIV,
            (t[2] * t[0] + t[3] * t[2]) % DIV,
            (t[2] * t[1] + t[3] * t[3]) % DIV
        };
    } else {
        std::vector<long long> t = fib((n-1)/2);
        return std::vector<long long> {
            (((t[0] * t[0] + t[1] * t[2]) % DIV) + ((t[0] * t[1] + t[1] * t[3]) % DIV)) % DIV,
            (t[0] * t[0] + t[1] * t[2]) % DIV,
            (((t[2] * t[0] + t[3] * t[2]) % DIV) + ((t[2] * t[1] + t[3] * t[3]) % DIV)) % DIV,
            (t[2] * t[0] + t[3] * t[2]) % DIV
        };
    }
}

int main()
{
    long long n;
    std::cin >> n;
    
    if(n == 0) {
        std::cout << 0;
        return 0;
    } else if(n == 1 || n == 2) {
        std::cout << 1;
        return 0;
    }
    
    std::vector<long long> res = fib(n-1);
    std::cout << res[0];

    return 0;
}