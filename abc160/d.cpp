#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, x, y;
    cin >> n >> x >> y;
    --x;
    --y;

    vector<int64_t> ans(n);
    for (auto i : irange(0L, n)) {
        for (auto j : irange(i + 1, n)) {
            int64_t k = min({j - i, abs(i - x) + abs(j - y) + 1,
                             abs(i - y) + abs(j - x) + 1});
            ++ans[k];
        }
    }

    for (auto i : irange(1L, n)) {
        cout << ans[i] << endl;
    }
}