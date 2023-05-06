#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, c;
    cin >> n >> c;

    vector<vector<int64_t>> u(2 * 100000L + 1, vector<int64_t>(c));
    for (auto i : irange(0L, n)) {
        int64_t s, t, c;
        cin >> s >> t >> c;
        --c;

        s *= 2;
        t *= 2;

        if (u[s][c] == 0) {
            u[s - 1][c] = 1;
        } else {
            ++u[s][c];
        }

        if (u[t - 1][c] == 1) {
            u[t - 1][c] = 0;
            ++u[t][c];
        }
        --u[t][c];
    }

    int64_t d = 0, ans = 0;
    for (const auto& uu : u) {
        d = accumulate(uu.begin(), uu.end(), d);

        ans = max(ans, d);
    }

    cout << ans << endl;
}