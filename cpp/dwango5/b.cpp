#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int n, k;
    cin >> n >> k;

    vector<int64_t> s(n + 1);
    for (auto i : irange(1, n + 1)) {
        int64_t a;
        cin >> a;
        s[i] = s[i-1] + a;
    }

    constexpr auto MAX_BIT = 40;
    vector<bitset<MAX_BIT>> sums;
    sums.reserve(n * (n+1) / 2);
    for (int l : irange(0, n)) {
        for (int r : irange(l, n)) {
            // sのindexずらしたので注意
            sums.push_back(s[r + 1] - s[l]);
            cerr << sums.back() << endl;
        }
    }

    int64_t ans = 0;
    for (int i : irange(MAX_BIT, -1, -1)) {
        int count = 0;
        for (auto s : sums) {
            count += (s[i] ? 1 : 0);
        }

        if (count < k) {
            continue;
        }

        ans += 1L << i;
        cerr << i << " " << ans << endl;
        decltype(sums) next;
        next.reserve(sums.size());
        for (const auto& s : sums) {
            if (s[i]) {
                next.push_back(s);
            }
        }
        sums.swap(next);
    }

    cout << ans << endl;
}