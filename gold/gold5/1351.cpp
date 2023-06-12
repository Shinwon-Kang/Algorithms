#include <iostream>
#include <stdio.h>
#include <map>

std::map<long long, long long> m;

long long rec(const long long& N, const long long& P, const long long& Q) {
    if(N == 0) {
        return 1;
    }
    
    if(m.find(N) != m.end()) {
        return m[N];
    }
    
    m[N] = rec(N/P, P, Q) + rec(N/Q, P, Q);
    return m[N];
}

int main()
{
    long long N, P, Q;
    scanf("%lld %lld %lld", &N, &P, &Q);
    
    long long r = rec(N, P, Q);
    
    std::cout << r;
    
    return 0;
}