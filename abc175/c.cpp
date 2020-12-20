#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t x, k, d;
    cin >> x >> k >> d;

    if (x < 0) {
        x = -x;
    }

    auto t = min(x / d, k);

    x -= t * d;
    k -= t;

    if (k % 2 == 0) {
        cout << x << endl;
    } else {
        cout << -(x - d) << endl;
    }
}