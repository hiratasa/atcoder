#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

bool check(int64_t n, int64_t s, int64_t b) {
    if (b == 1) {
        return false;
    }

    int64_t ss = 0;

    while (n > 0) {
        ss += n % b;
        n /= b;
    }

    return s == ss;
}

int main() {
    int64_t n, s;
    cin >> n >> s;

    if (s > n) {
        cout << -1 << endl;
        return 0;
    }

    for (auto b : irange(2L, (int64_t)(sqrt(n)) + 1)) {
        if (check(n, s, b)) {
            cout << b << endl;
            return 0;
        }
    }

    // xb + y = n
    // x + y = s
    // => x*(b-1) = n - s
    auto m = n - s;
    int64_t b = -1L;
    for (auto k = 2L; k * k <= m; ++k) {
        if (m % k == 0) {
            if (check(n, s, k + 1)) {
                cout << k + 1 << endl;
                return 0;
            }
            if (check(n, s, m / k + 1)) {
                b = m / k + 1;
            }
        }
    }

    if (b > 0) {
        cout << b << endl;
        return 0;
    }

    if (check(n, s, m + 1)) {
        cout << m + 1 << endl;
        return 0;
    }

    if (s == n) {
        cout << n + 1 << endl;
        return 0;
    }

    cout << -1 << endl;
}