#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    int64_t s = 0;
    for (auto&& aa : a) {
        cin >> aa;
        aa *= 2;
        s += aa;
    }

    auto m = s / 2;
    auto d = s;
    auto ss = 0L;
    for (auto aa : a | sliced(0, n - 1)) {
        ss += aa;
        d = min(d, abs(ss - m));
    }

    cout << d << endl;
}