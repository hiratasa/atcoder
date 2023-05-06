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
    int64_t b = 1;
    for (auto&& aa : a) {
        cin >> aa;
        s += b * aa;
        b *= -1;
    }

    cout << s;
    for (auto i : irange(0L, n - 1)) {
        auto aa = a[i];
        s = 2 * (aa - s / 2);
        cout << " " << s;
    }
    cout << endl;
}
