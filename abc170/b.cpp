#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t x, y;
    cin >> x >> y;

    // a + b = x
    // 2a + 4b = y
    // => b = y/2 - x
    //    a = 2x - y/2
    if (y % 2 != 0 || y / 2 - x < 0 || 2 * x - y / 2 < 0) {
        cout << "No" << endl;
    } else {
        cout << "Yes" << endl;
    }
}