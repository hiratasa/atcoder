#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, m;
    cin >> n >> m;

    int64_t l = 1, r = n;
    for (auto _ : irange(0L, m)) {
        int64_t ll, rr;
        cin >> ll >> rr;
        l = max(l, ll);
        r = min(r, rr);
    }

    cout << max(r - l + 1, 0L) << endl;
}