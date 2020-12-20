#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    array<array<int64_t, 10>, 10> a{};
    int64_t d = 1;
    for (auto i : irange(1L, n + 1)) {
        if (i >= d * 10) {
            d *= 10;
        }

        auto s = i / d;
        auto e = i % 10;
        ++a[s][e];
    }

    int64_t ans = 0;
    for (auto i : irange(1L, 10L)) {
        for (auto j : irange(1L, 10L)) {
            ans += a[i][j] * a[j][i];
        }
    }

    cout << ans << endl;
}