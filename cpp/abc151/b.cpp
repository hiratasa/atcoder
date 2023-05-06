#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k, m;
    cin >> n >> k >> m;

    auto ans = m * n;
    for (auto i : irange(0L, n - 1)) {
        int64_t a;
        cin >> a;
        ans -= a;
    }

    if (ans > k) {
        cout << -1 << endl;
    } else {
        cout << max(ans, 0L) << endl;
    }
}