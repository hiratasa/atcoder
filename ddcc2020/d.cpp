#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t m;
    cin >> m;

    int64_t s = 0, n = 0;
    for (auto _ : irange(0L, m)) {
        int64_t c, d;
        cin >> d >> c;

        s += d * c;
        n += c;
    }

    cout << n - 1 + (s - 1) / 9 << endl;
}