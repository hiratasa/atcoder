#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, a, b;
    cin >> n >> a >> b;

    cout << min(a, b) << " " << max(a + b - n, 0L) << endl;
}