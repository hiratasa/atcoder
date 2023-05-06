#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n), b(n);
    for (auto&& aa : a) {
        cin >> aa;
    }
    for (auto&& bb : b) {
        cin >> bb;
    }

    for (auto i : irange(0L, n)) {
        if (b[i] != a[i] && 2 * b[i] >= a[i]) {
            cout << -1 << endl;
            return 0;
        }
    }

    vector<vector<vector<bool>>> table(
            51, vector<vector<bool>>(51, vector<bool>(51)));
    for (auto i : irange(0L, 51L)) {
        table[0][i][i] = true;
    }
    for (auto i : irange(1L, 51L)) {
        for (auto j : irange(0L, 51L)) {
            for (auto k : irange(0L, 51L)) {
                // i以下の数のみでj->kにできるか
                table[i][j][k] = (table[i - 1][j][k] || table[i - 1][j % i][k]);
            }
        }
    }

    vector<vector<bool>> c(n, vector<bool>(51));
    for (auto i : irange(0L, n)) {
        c[i][a[i]] = true;
    }
    int64_t ans = 0;
    for (auto i : irange(1L, 51L) | reversed) {
        bool required = false;
        for (auto j : irange(0L, n)) {
            bool j_required = true;
            for (auto k : irange(0L, 51L)) {
                if (!c[j][k]) {
                    continue;
                }

                if (!table[i][k][b[j]]) {
                    c[j][k] = false;
                    continue;
                }

                if (table[i - 1][k][b[j]]) {
                    j_required = false;
                }
            }
            required |= j_required;
        }

        if (!required) {
            continue;
        }

        ans += 1L << i;
        for (auto j : irange(0L, n)) {
            for (auto k : irange(0L, 51L)) {
                if (!c[j][k]) {
                    continue;
                }

                if (table[i - 1][k % i][b[j]]) {
                    c[j][k % i] = true;
                }
            }
        }
    }

    cout << ans << endl;
}