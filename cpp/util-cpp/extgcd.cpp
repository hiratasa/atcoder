#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

// a * x + b * y = gcd(a, b)
// xにb/gを足してyに-a/gを足したものも解
int64_t extgcd(int64_t a, int64_t b, int64_t& x, int64_t& y) {
    if (b == 0) {
        x = 1;
        y = 0;
        return a;
    }

    int64_t g = extgcd(b, a % b, y, x);
    y -= a / b * x;
    return g;
}