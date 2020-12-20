#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, a, b, c, d;
    cin >> n >> a >> b >> c >> d;

    b -= a;
    if (b < 0) {
        b *= -1;
    }

    if (b > d * (n - 1)) {
        cout << "NO" << endl;
        return 0;
    }

    int64_t l = b, u = b;
    for (auto i : irange(1L, n) | reversed) {
        if (c * i <= u && l <= d * i) {
            cout << "YES" << endl;
            return 0;
        }

        l += c;
        u += d;
    }

    cout << "NO" << endl;
}