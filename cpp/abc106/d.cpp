#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m, Q;
    cin >> n >> m >> Q;

    vector<vector<int64_t>> lrs(n + 1, vector<int64_t>(n + 1));
    for (auto i : irange(0L, m)) {
        int64_t l, r;
        cin >> l >> r;

        ++lrs[l][r];
    }

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, n + 1)) {
            lrs[i + 1][j] += lrs[i][j];
        }
    }

    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, n + 1)) {
            lrs[j][i + 1] += lrs[j][i];
        }
    }

    for (auto _ : irange(0L, Q)) {
        int64_t p, q;
        cin >> p >> q;

        cout << lrs[n][q] - lrs[p - 1][q] << "\n";
    }
}