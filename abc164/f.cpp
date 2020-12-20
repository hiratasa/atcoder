#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> s(n), t(n);
    vector<bitset<64>> u(n), v(n);

    for (auto i : irange(0L, n)) {
        cin >> s[i];
    }
    for (auto i : irange(0L, n)) {
        cin >> t[i];
    }
    for (auto i : irange(0L, n)) {
        uint64_t tmp;
        cin >> tmp;
        u[i] = tmp;
    }
    for (auto i : irange(0L, n)) {
        uint64_t tmp;
        cin >> tmp;
        v[i] = tmp;
    }

    vector ans(n, vector(n, bitset<64>()));
    for (auto i : irange(0L, 64L)) {
        array<array<bool, 2>, 2> has{};
        array fix{vector<bool>(n), vector<bool>(n)};
        for (auto j : irange(0L, n)) {
            if (s[j] != u[j][i]) {
                has[0][u[j][i]] = true;
                fix[0][j] = true;
                for (auto k : irange(0L, n)) {
                    ans[j][k][i] = u[j][i];
                }
            }
        }
        for (auto j : irange(0L, n)) {
            if (t[j] != v[j][i]) {
                has[1][v[j][i]] = true;
                fix[1][j] = true;
                if (has[0][(v[j][i] + 1) % 2]) {
                    cout << -1 << endl;
                    return 0;
                }
                for (auto k : irange(0L, n)) {
                    ans[k][j][i] = v[j][i];
                }
            }
        }

        vector<int64_t> tmp1, tmp2;
        for (auto j : irange(0L, n)) {
            if (s[j] == u[j][i]) {
                bool ok = has[1][u[j][i]];
                bool ng = (!ok && all_of(fix[1].begin(), fix[1].end(),
                                         [](bool b) { return b; }));

                if (ng) {
                    cout << -1 << endl;
                    return 0;
                }
                tmp1.push_back(j);
            }
        }
        for (auto j : irange(0L, n)) {
            if (t[j] == v[j][i]) {
                bool ok = has[0][v[j][i]];
                bool ng = (!ok && all_of(fix[0].begin(), fix[0].end(),
                                         [](bool b) { return b; }));

                if (ng) {
                    cout << -1 << endl;
                    return 0;
                }
                tmp2.push_back(j);
            }
        }

        if (tmp1.empty() || tmp2.empty()) {
            continue;
        }

        if (tmp1.size() >= 2 && tmp2.size() >= 2) {
            for (auto j : irange(0uL, max(tmp1.size(), tmp2.size()))) {
                ans[tmp1[j % tmp1.size()]][tmp2[j % tmp2.size()]][i] = 1;
            }
        } else if (tmp1.size() == 1) {
            auto j = tmp1[0];
            if (!has[1][u[j][i]] &&
                all_of(tmp2.begin(), tmp2.end(),
                       [&](int64_t k) { return !has[0][v[k][i]]; }) &&
                all_of(tmp2.begin(), tmp2.end(),
                       [&](int64_t k) { return v[k][i] != u[j][i]; })) {
                cout << -1 << endl;
                return 0;
            }
            for (auto k : tmp2) {
                if (has[0][v[k][i]]) {
                    ans[j][k][i] = u[j][i];
                } else {
                    ans[j][k][i] = v[k][i];
                }
            }
        } else if (tmp2.size() == 1) {
            auto j = tmp2[0];
            if (!has[0][v[j][i]] &&
                all_of(tmp1.begin(), tmp1.end(),
                       [&](int64_t k) { return !has[1][u[k][i]]; }) &&
                all_of(tmp1.begin(), tmp1.end(),
                       [&](int64_t k) { return u[k][i] != v[j][i]; })) {
                cout << -1 << endl;
                return 0;
            }
            for (auto k : tmp1) {
                if (has[1][u[k][i]]) {
                    ans[k][j][i] = v[j][i];
                } else {
                    ans[k][j][i] = u[k][i];
                }
            }
        }
    }

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, n)) {
            cout << ans[i][j].to_ulong() << " ";
        }
        cout << "\n";
    }
}