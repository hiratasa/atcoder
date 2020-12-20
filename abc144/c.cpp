#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    int64_t ans = numeric_limits<int64_t>::max();
    for (int64_t k = 1; k * k <= n; ++k) {
        if (n % k == 0) {
            auto l = n / k;

            ans = (k - 1) + (l - 1);
        }
    }

    cout << ans << endl;
}