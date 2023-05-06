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

    int64_t m0 = a[0] + a[1], m1 = -a[0] - a[1];
    for (auto i : irange(2L, n)) {
        tie(m0, m1) =
                make_pair(max(m0, m1) + a[i],
                          max(m0 - 2 * a[i - 1], m1 + 2 * a[i - 1]) - a[i]);
    }

    cout << max(m0, m1) << endl;
}