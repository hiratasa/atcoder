#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<int64_t>> s(m);
    for (auto&& ss : s) {
        int64_t k;
        cin >> k;
        ss.resize(k);
        for (auto&& sss : ss) {
            cin >> sss;
            --sss;
        }
    }

    vector<int64_t> p(m);
    for (auto&& pp : p) {
        cin >> pp;
    }

    int64_t ans = 0;
    for (uint64_t b = 0; b < (1L << n); ++b) {
        bitset<10> flags(b);

        auto r = irange(0L, m);
        auto ok = all_of(r.begin(), r.end(), [&](int64_t i) {
            auto nums = count_if(s[i].begin(), s[i].end(), [&flags] (int64_t idx) {
                return flags[idx];
            });

            return nums % 2 == p[i];
        });

        if (ok) {
            ++ans;
        }
    }

    cout << ans << endl;
}