#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t r, d, x;
    cin >> r >> d >> x;

    for(auto _ : irange(0, 10)) {
        x = r * x - d;
        cout << x << endl;
    }
}