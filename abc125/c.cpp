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
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> l(n), r(n);
    l[0] = a[0];
    r[n - 1] = a[n - 1];
    for (auto i : irange(0L, n - 1)) {
        l[i + 1] = gcd(l[i], a[i + 1]);
        r[n - i - 2] = gcd(r[n - i - 1], a[n - i - 2]);
    }

    int64_t ans = max(l[n - 2], r[1]);
    for (auto i : irange(1L, n - 1)) {
        ans = max(ans, gcd(l[i - 1], r[i + 1]));
    }

    cout << ans << endl;
}