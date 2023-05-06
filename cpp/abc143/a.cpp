#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t a, b;
    cin >> a >> b;

    cout << max(a - 2 * b, 0L) << endl;
}