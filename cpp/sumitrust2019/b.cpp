#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    int64_t x = n / 1.08;
    while ((int)(x * 1.08) < n) {
        ++x;
    }

    if ((int)(x * 1.08) == n) {
        cout << x << endl;
    } else {
        cout << ":(" << endl;
    }
}