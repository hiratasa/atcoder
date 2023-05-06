#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, k;
    cin >> n >> k;

    if ((n + 1) / 2 >= k) {
        cout << "YES" << endl;
    } else {
        cout << "NO" << endl;
    }
}