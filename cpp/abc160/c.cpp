#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t k, n;
    cin >> k >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    int64_t ans = k - (k + a[0] - a[n - 1]);
    for (auto i : irange(0L, n - 1)) {
        ans = min(ans, k - (a[i + 1] - a[i]));
    }

    cout << ans << endl;
}