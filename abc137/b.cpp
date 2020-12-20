#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n = 1000000;

    int64_t k, x;
    cin >> k >> x;

    const auto* delim = "";
    for (auto i = max(-n, x - k + 1); i <= min(n, x + k - 1); ++i) {
        cout << delim << i;
        delim = " ";
    }
    cout << endl;
}