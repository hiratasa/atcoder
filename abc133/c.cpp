#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t l, r;
    cin >> l >> r;

    if (max(0L, l - 1) / 2019 != r / 2019) {
        cout << 0 << endl;
        return 0;
    }

    int64_t m = 2019;
    for (auto i : irange(l, r + 1)) {
        for (auto j : irange(i + 1, r + 1)) {
            m = min(m, (i * j) % 2019);
        }
    }

    cout << m << endl;
}
