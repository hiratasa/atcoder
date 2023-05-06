#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, k;
    cin >> n >> k;

    int64_t ans = 0;
    for (auto _ : irange(0L, n)) {
        int64_t h;
        cin >> h;

        if (h >= k) {
            ++ans;
        }
    }

    cout << ans << endl;
}