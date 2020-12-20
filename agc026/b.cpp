#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t gcd(int64_t x, int64_t y) {
    if (x == 0) {
        return y;
    }

    return gcd(y % x, x);
}

bool solve(int64_t a, int64_t b, int64_t c, int64_t d) {
    if (a < b) {
        return false;
    }

    if (d < b) {
        return false;
    }

    if (b <= c + 1) {
        return true;
    }

    // c + 1 < b

    a %= b;

    if (a > c) {
        return false;
    }

    auto g = gcd(b, d);

    if (g == b) {
        return true;
    }

    return (c - a) / g == ((b - 1 - a)) / g;
}

int main() {
    int64_t t;
    cin >> t;

    for (auto _ : irange(0L, t)) {
        int64_t a, b, c, d;
        cin >> a >> b >> c >> d;

        if (solve(a, b, c, d)) {
            cout << "Yes" << endl;
        } else {
            cout << "No" << endl;
        }
    }
}