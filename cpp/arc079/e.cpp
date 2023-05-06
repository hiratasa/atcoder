#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    int64_t ans = 0;
    while (true) {
        int64_t k = 0;

        for (auto&& aa : a) {
            if (aa > n - 1) {
                auto t = (aa - (n - 1) - 1) / n + 1;
                k += t;
                aa -= (n + 1) * t;
                assert(aa <= n - 1);
            }
        }
        bool ok = true;
        for (auto&& aa : a) {
            aa += k;
            if (aa > n - 1) {
                ok = false;
            }
        }

        ans += k;
        if (ok) {
            cout << ans << endl;
            return 0;
        }
    }
}