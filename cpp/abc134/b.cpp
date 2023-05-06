#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, d;
    cin >> n >> d;

    cout << (n - 1) / (2 * d + 1) + 1 << endl;
}