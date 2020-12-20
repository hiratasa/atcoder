#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t a, b, c;
    cin >> a >> b >> c;

    if (c <= a + b) {
        cout << b + c << endl;
    } else {
        cout << b + (a + b + 1) << endl;
    }
}