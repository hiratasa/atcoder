#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

bool exec(int64_t x, int64_t i) {
    // a^5 - b^5 = x, a - b = i
    auto r = irange(0L, 1000L);
    auto b = *partition_point(r.begin(), r.end(), [&](int64_t b) {
        auto a = b + i;
        return a * a * a * a * a - b * b * b * b * b < x;
    });
    auto a = b + i;
    if (a * a * a * a * a - b * b * b * b * b == x) {
        cout << a << " " << b << endl;
        return true;
    }

    // a^5 + b^5 = x, a + b = i
    for (auto b :
         irange(0L, min(i / 2 + 1, (int64_t)(pow((double)x, 0.2) + 1)))) {
        auto a = i - b;

        if (a * a * a * a * a + b * b * b * b * b == x) {
            cout << a << " " << -b << endl;
            return true;
        }
    }

    return false;
}

int main() {
    int64_t x;
    cin >> x;

    for (int64_t i = 1; i * i <= x; ++i) {
        if (x % i > 0) {
            continue;
        }

        if (exec(x, i)) {
            return 0;
        }

        if (exec(x, x / i)) {
            return 0;
        }
    }
}