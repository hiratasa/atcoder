#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    int64_t ans = numeric_limits<int64_t>::max();
    for (auto sgn : {1L, -1L}) {
        int64_t s = 0;
        int64_t t = 0;
        for (auto aa : a) {
            s += aa;
            if (s * sgn <= 0) {
                t += -(s * sgn) + 1;
                s = sgn;
            }
            sgn *= -1L;
        }
        ans = min(ans, t);
    }

    cout << ans << endl;
}