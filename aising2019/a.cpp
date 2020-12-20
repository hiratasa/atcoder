#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, h, w;
    cin >> n >> h >> w;

    cout << (n - h + 1) * (n - w + 1) << endl;
}