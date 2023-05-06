#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    int64_t m = numeric_limits<int64_t>::max();
    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        int64_t p;
        cin >> p;

        if (p < m) {
            ++ans;
        }

        m = min(m, p);
    }

    cout << ans << endl;
}