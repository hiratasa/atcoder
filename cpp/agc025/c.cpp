#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> l(n), r(n);
    for (auto i : irange(0L, n)) {
        cin >> l[i] >> r[i];
    }

    sort(l.rbegin(), l.rend());
    sort(r.begin(), r.end());

    int64_t ans = 0;
    int64_t s = 0;
    for (auto i : irange(0L, n)) {
        ans = max({ans, s + l[i], s - r[i], s + l[i] - r[i]});
        s += l[i] - r[i];
    }

    ans *= 2;

    cout << ans << endl;
}