#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t a, p;
    cin >> a >> p;

    p += 3 * a;

    cout << p / 2 << endl;
}