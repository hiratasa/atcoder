#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;

main() {
    int64_t a, b;
    cin >> a >> b;

    if (a >= 13) {
        cout << b << endl;
    } else if (a >= 6) {
        cout << b / 2 << endl;
    } else {
        cout << 0 << endl;
    }
}