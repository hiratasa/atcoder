#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w, k;
    cin >> h >> w >> k;

    vector<vector<bool>> m(h);
    for (auto&& r : m) {
        string s;
        cin >> s;

        for (auto c : s) {
            r.push_back(c == '#');
        }
    }

    int64_t ans = 0;
    for (auto s : irange(0uL, 1uL << (h + w))) {
        bitset<6> hb(s & ((1uL << h) - 1));
        bitset<6> wb(s >> h);

        int64_t t = 0;
        for (auto i : irange(0L, h)) {
            if (hb[i]) {
                continue;
            }
            for (auto j : irange(0L, w)) {
                if (wb[j]) {
                    continue;
                }

                if (m[i][j]) {
                    ++t;
                }
            }
        }

        if (t == k) {
            ++ans;
        }
    }

    cout << ans << endl;
}