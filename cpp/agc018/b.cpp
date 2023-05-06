#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<int64_t>> a(n, vector<int64_t>(m));
    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, m)) {
            cin >> a[i][m - j - 1];
            --a[i][m - j - 1];
        }
    }

    int64_t ans = n;
    vector<bool> available(m, true);
    for (auto _ : irange(0L, m)) {
        vector<int64_t> c(m);

        for (const auto& b :
             a | transformed([](const auto& aa) { return aa.back(); })) {
            ++c[b];
        }

        auto b = max_element(c.begin(), c.end()) - c.begin();
        available[b] = false;
        ans = min(ans, c[b]);

        for (auto&& aa : a) {
            while (!available[aa.back()]) {
                aa.pop_back();
            }
        }
    }

    cout << ans << endl;
}