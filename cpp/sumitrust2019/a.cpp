#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t m1, d1, m2, d2;
    cin >> m1 >> d1 >> m2 >> d2;

    if (m1 != m2) {
        cout << 1 << endl;
    } else {
        cout << 0 << endl;
    }
}