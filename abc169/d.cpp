#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    int64_t ans = 0;
    for (int64_t i = 2; i * i <= n; ++i) {
        if (n % i == 0) {
            int64_t t = 0;
            while (n % i == 0) {
                n /= i;
                ++t;
            }

            // cerr << i << "," << t << endl;
            int64_t j = 1;
            while (t - j >= 0) {
                t -= j;
                ++j;
                ++ans;
            }
        }
    }

    if (n != 1) {
        ++ans;
    }

    cout << ans << endl;
}