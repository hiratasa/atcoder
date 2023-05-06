#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t a, b, c;
    cin >> a >> b >> c;

    cout << max(c - (a - b), 0L) << endl;
}