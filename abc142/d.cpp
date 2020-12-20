#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t gcd(int64_t a, int64_t b) {
    if (a < b) {
        return gcd(b, a);
    }

    if (b == 0) {
        return a;
    }

    return gcd(b, a % b);
}

main() {
    int64_t a, b;
    cin >> a >> b;

    auto g = gcd(a, b);

    int64_t ans = 1;
    for (int64_t p = 2; p * p <= g; ++p) {
        if (g % p == 0) {
            ++ans;
        }

        while (g % p == 0) {
            g /= p;
        }
    }

    if (g > 1) {
        ++ans;
    }

    cout << ans << endl;
}