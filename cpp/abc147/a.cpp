#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t a, b, c;
    cin >> a >> b >> c;

    if (a + b + c >= 22) {
        cout << "bust" << endl;
    } else {
        cout << "win" << endl;
    }

    return 0;
}