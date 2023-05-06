#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    if (n % 2 == 0) {
        cout << 0.5 << endl;
    } else {
        cout << setprecision(10) << ((n - 1) / 2 + 1) / (double)n << endl;
    }
}