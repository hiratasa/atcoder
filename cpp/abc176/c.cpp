#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector a(n, 0L);
    for (auto&& aa : a) {
        cin >> aa;
    }

    int64_t mx = 0;
    int64_t ans = 0;
    for (auto aa : a) {
        ans += max(mx - aa, 0L);
        mx = max(mx, aa);
    }

    cout << ans << endl;
}