#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t gcd(int64_t x, int64_t y) {
    if (x > y) {
        swap(x, y);
    }

    if (x == 0) {
        return y;
    }

    return gcd(y % x, x);
}

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> a(n);
    for (auto i : irange(0L, n)) {
        cin >> a[i];
    }

    int64_t lcm = 1;
    for (auto i : irange(0L, n)) {
        auto g = gcd(lcm, a[i] / 2);
        lcm *= (a[i] / 2) / g;

        if (lcm > m) {
            cout << 0 << endl;
            return 0;
        }
    }

    for (auto i : irange(0L, n)) {
        if (lcm / (a[i] / 2) % 2 == 0) {
            cout << 0 << endl;
            return 0;
        }
    }

    cout << m / lcm - m / (2 * lcm) << endl;
}