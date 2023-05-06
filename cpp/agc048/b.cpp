#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector a(n, 0L), b(n, 0L);
    for (auto&& aa : a) {
        cin >> aa;
    }
    for (auto&& bb : b) {
        cin >> bb;
    }

    int64_t s = 0;
    vector c(n, 0L);
    for (auto i : irange(0L, n)) {
        if (i % 2 == 0) {
            s += a[i];
            c[i] = b[i] - a[i];
        } else {
            s += b[i];
            c[i] = a[i] - b[i];
        }
    }

    sort(c.rbegin(), c.rend());
    c.resize(n / 2);
    for (auto cc : c) {
        s += cc;
    }

    cout << s << endl;
}