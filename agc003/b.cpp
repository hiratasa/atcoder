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
    int64_t m = 0;
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;
        m += a;

        if (a == 0) {
            ans += m / 2;
            m = 0;
        }
    }

    ans += m / 2;

    cout << ans << endl;
}