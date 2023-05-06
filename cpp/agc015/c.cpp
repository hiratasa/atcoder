#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m, q;
    cin >> n >> m >> q;

    vector<vector<bool>> s(n + 1, vector<bool>(m + 1));
    for (auto i : irange(0L, n)) {
        bitset<2001> bs;
        cin >> bs;

        // bsは順序が逆
        for (auto j : irange(0L, m)) {
            s[i + 1][j + 1] = bs[m - j - 1];
        }
    }

    vector<vector<int64_t>> t(n + 1, vector<int64_t>(m + 1));
    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, m)) {
            t[i + 1][j + 1] = t[i][j + 1] + t[i + 1][j] - t[i][j];
            if (s[i + 1][j + 1] && !s[i + 1][j] && !s[i][j + 1]) {
                ++t[i + 1][j + 1];
            }
            if (s[i + 1][j + 1] && s[i + 1][j] && s[i][j + 1]) {
                --t[i + 1][j + 1];
            }

            // cerr << "t[" << i + 1 << "][" << j + 1 << "] = " << t[i + 1][j +
            // 1]
            //      << endl;
        }
    }

    vector<vector<int64_t>> bx(n + 1, vector<int64_t>(m + 1)),
            by(n + 1, vector<int64_t>(m + 1));
    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, m)) {
            bx[i + 1][j + 1] = bx[i + 1][j];
            if (s[i + 1][j + 1] && s[i][j + 1]) {
                ++bx[i + 1][j + 1];
            }

            by[i + 1][j + 1] = by[i][j + 1];
            if (s[i + 1][j + 1] && s[i + 1][j]) {
                ++by[i + 1][j + 1];
            }

            // cerr << "bx[" << i + 1 << "][" << j + 1
            //      << "] = " << bx[i + 1][j + 1] << endl;
            // cerr << "by[" << i + 1 << "][" << j + 1
            //      << "] = " << by[i + 1][j + 1] << endl;
        }
    }

    for (auto _ : irange(0L, q)) {
        int64_t x1, y1, x2, y2;
        cin >> x1 >> y1 >> x2 >> y2;

        cout << t[x2][y2] - t[x1 - 1][y2] - t[x2][y1 - 1] + t[x1 - 1][y1 - 1] +
                        bx[x1][y2] - bx[x1][y1 - 1] + by[x2][y1] -
                        by[x1 - 1][y1]
             << "\n";
    }
}