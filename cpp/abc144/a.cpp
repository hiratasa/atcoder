#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t a, b;
    cin >> a >> b;

    if (a <= 9 && b <= 9) {
        cout << a * b << endl;
    } else {
        cout << -1 << endl;
    }
}