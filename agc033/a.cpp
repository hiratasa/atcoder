#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t h, w;
    cin >> h >> w;

    vector<vector<int64_t>> m(h);
    for (auto&& row : m) {
        string s;
        cin >> s;
        row.reserve(w);
        for (auto c : s) {
            if (c == '#') {
                row.push_back(0);
            } else {
                row.push_back(numeric_limits<int>::max());
            }
        }
    }


    for (auto i = 0; i < h; ++i) {
        int64_t current = 2000;
        for (int j = 0; j < w; ++j, ++current) {
            current = min(m[i][j], current);
            m[i][j] = current;
        }

        current = 2000;
        for (int j = w - 1; j >= 0; --j, ++current) {
            current = min(m[i][j], current);
            m[i][j] = current;
        }
    }

    for (auto j = 0; j < w; ++j) {
        int64_t current = 2000;
        for (int i = 0; i < h; ++i, ++current) {
            current = min(m[i][j], current);
            m[i][j] = current;
        }

        current = 2000;
        for (int i = h - 1; i >= 0; --i, ++current) {
            current = min(m[i][j], current);
            m[i][j] = current;
        }
    }

    int64_t ans = 0;
    for (auto i = 0; i < h; ++i) {
        for (auto j = 0; j < w; ++j) {
            ans = max(ans, m[i][j]);
        }
    }

    cout << ans << endl;
}
