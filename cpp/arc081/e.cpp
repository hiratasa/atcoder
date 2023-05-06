#include <bits/stdc++.h>
#include <boost/range/adaptor/reversed.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    string a;
    cin >> a;

    constexpr int64_t M = 'z' - 'a' + 1;

    vector<vector<int64_t>> idxs(a.size() + 1, vector<int64_t>(M, -1));
    for (auto i : irange(0uL, a.size())) {
        for (auto c : irange('a' + 0, 'z' + 1)) {
            if (a[i] == c) {
                idxs[i + 1][c - 'a'] = i;
            } else {
                idxs[i + 1][c - 'a'] = idxs[i][c - 'a'];
            }
        }
    }

    vector<vector<int64_t>> idxs2(a.size() + 1, vector<int64_t>(M, -1));
    int64_t cur = a.size();
    for (auto i : irange(1uL, a.size() + 1)) {
        int64_t m = a.size();
        for (auto c : irange(0L, M)) {
            auto idx = idxs[cur][c];
            if (idx == -1) {
                string s;
                s += c + 'a';

                auto ch = c;
                int64_t idx3 = -1;
                for (auto j : irange(1uL, i) | reversed) {
                    for (auto d : irange(0L, M)) {
                        if (idxs[idxs2[j][d]][ch] == idx3) {
                            s += d + 'a';
                            ch = d;
                            idx3 = idxs2[j][d];
                            break;
                        }
                    }
                }

                cout << s << endl;
                return 0;
            }

            if (idx < m) {
                m = idx;
            }

            idxs2[i][c] = idx;
        }

        cur = m;
    }
}