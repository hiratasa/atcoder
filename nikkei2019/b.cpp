#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    string a, b, c;
    cin >> a >> b >> c;

    int64_t count = 0;
    for (auto i : irange(0L, n)) {
        auto ca = a[i];
        auto cb = b[i];
        auto cc = c[i];

        if (ca == cb && cb == cc) {
            continue;
        }

        count += 2;
        if (ca == cb || cb == cc || cc == ca) {
            --count;
        }
    }

    cout << count << endl;
}