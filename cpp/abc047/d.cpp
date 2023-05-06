#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, t;
    cin >> n >> t;

    vector<int64_t> a(n);
    int64_t m = 1L << 30, g = 0;
    for (auto&& aa : a) {
        cin >> aa;
        m = min(m, aa);
        g = max(g, aa - m);
    }

    m = 1L << 30;
    int64_t ans = 0;
    for (auto aa : a) {
        m = min(m, aa);

        if (aa - m == g) {
            ++ans;
        }
    }

    cout << ans << endl;
}