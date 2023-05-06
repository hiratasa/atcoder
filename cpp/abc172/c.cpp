#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m, k;
    cin >> n >> m >> k;

    vector a(n, 0L), b(m, 0L);
    for (auto&& aa : a) {
        cin >> aa;
    }
    for (auto&& bb : b) {
        cin >> bb;
    }

    vector sa(1, 0L), sb(1, 0L);
    for (auto aa : a) {
        sa.push_back(sa.back() + aa);
    }
    for (auto bb : b) {
        sb.push_back(sb.back() + bb);
    }

    int64_t ans = 0;
    for (auto i : irange(0L, n + 1)) {
        int64_t s = sa[i];

        if (s > k) {
            break;
        }

        int64_t t =
                i + (upper_bound(sb.begin(), sb.end(), k - s) - sb.begin() - 1);
        ans = max(ans, t);
    }

    cout << ans << endl;
}