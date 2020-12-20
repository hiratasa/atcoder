#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t gcd(int64_t a, int64_t b) {
    if (a == 0) {
        return b;
    }

    return gcd(b % a, a);
}

int main() {
    int64_t x, y;
    char c;
    cin >> x >> c >> y;

    auto g = gcd(x, y);
    x /= g;
    y /= g;

    // (n - 1) / 2 <= x/y <= (n + 1) / 2
    // 2x/y - 1 <= n <= 2x/y + 1
    int64_t k = 2 * x / y;

    bool ok = false;
    for (auto n : irange(max(1L, k - 1), k + 2)) {
        // x/y = (n(n + 1)/2 - m) / n
        // m = n(n + 1)/2 - n*(x/y)
        cerr << x << " " << y << " " << n << endl;
        if (n % y > 0) {
            continue;
        }
        int64_t m = n * (n + 1) / 2 - n / y * x;
        if (!(1 <= m && m <= n)) {
            continue;
        }
        cout << n << " " << m << endl;
        ok = true;
    }

    if (!ok) {
        cout << "Impossible" << endl;
    }
}