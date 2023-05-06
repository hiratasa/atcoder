#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w, n;
    cin >> h >> w >> n;

    unordered_map<pair<int64_t, int64_t>, int64_t,
                  boost::hash<pair<int64_t, int64_t>>>
            s;
    for (auto _ : irange(0L, n)) {
        int64_t a, b;
        cin >> a >> b;

        for (auto dx : {-2L, -1L, 0L}) {
            for (auto dy : {-2L, -1L, 0L}) {
                auto nx = a + dx;
                auto ny = b + dy;

                if (nx <= 0 || nx > h - 2L) {
                    continue;
                }

                if (ny <= 0 || ny > w - 2L) {
                    continue;
                }

                ++s[make_pair(nx, ny)];
            }
        }
    }

    array<int64_t, 10> ans{};
    ans[0] = (h - 2) * (w - 2);
    for (const auto& kv : s) {
        // cerr << kv.first.first << "," << kv.first.second << ":" << kv.second
        //      << endl;
        ++ans[kv.second];
        --ans[0L];
    }

    for (auto i : irange(0L, 10L)) {
        cout << ans[i] << endl;
    }
}