#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    int64_t ans = 0;
    vector f(n + 1, 0L);
    for (auto k : irange(1L, n + 1)) {
        for (int64_t m = k; m <= n; m += k) {
            ++f[m];
        }

        ans += k * f[k];
    }

    cout << ans << endl;
}