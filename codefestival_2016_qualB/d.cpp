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

    int64_t ans = 0;
    int64_t p = 1;
    for (auto i : irange(0L, n)) {
        ans += (a[i] - 1) / p;
        if (a[i] == p) {
            p = a[i] + 1;
        } else {
            p = max(2L, p);
        }
    }

    cout << ans << endl;
}