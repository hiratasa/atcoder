#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b;
    cin >> a >> b;

    vector<vector<bool>> ans;

    for (auto _ : irange(0L, max(0L, (a - 2)) / 50)) {
        ans.resize(ans.size() + 2, vector<bool>(100L, true));
        for (auto j : irange(0L, 50L)) {
            ans[ans.size() - 2][2 * j] = false;
        }
    }

    ans.resize(ans.size() + 2, vector<bool>(100L, true));
    auto d = a == 1 ? 0 : ((a - 2) % 50L + 1);
    for (auto j : irange(0L, d)) {
        ans[ans.size() - 2][2 * j] = false;
    }

    ans.resize(ans.size() + 1, vector<bool>(100L));

    for (auto _ : irange(0L, max(0L, (b - 2)) / 50)) {
        ans.resize(ans.size() + 2, vector<bool>(100L));
        for (auto j : irange(0L, 50L)) {
            ans[ans.size() - 2][2 * j] = true;
        }
    }

    ans.resize(ans.size() + 2, vector<bool>(100L));
    auto e = b == 1 ? 0 : ((b - 2) % 50L + 1);
    for (auto j : irange(0L, e)) {
        ans[ans.size() - 2][2 * j] = true;
    }

    cout << ans.size() << " " << ans[0].size() << endl;
    for (const auto& line : ans) {
        for (auto b : line) {
            if (b) {
                cout << '#';
            } else {
                cout << '.';
            }
        }

        cout << "\n";
    }
}