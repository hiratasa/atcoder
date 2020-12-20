#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    // sum[l,r] (r - l + 1)
    int64_t ans = n * (n * n + 3 * n + 2) / 6;

    for (auto _ : irange(0L, n - 1)) {
        int64_t u, v;
        cin >> u >> v;

        if (u > v) {
            swap(u, v);
        }

        ans -= u * (n - v + 1);
    }

    cout << ans << endl;
}