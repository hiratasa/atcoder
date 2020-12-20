#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t a, b;
    cin >> a >> b;

    for (auto i : irange(0L, b + 1)) {
        if (i * (a - 1) + 1 >= b) {
            cout << i << endl;
            return 0;
        }
    }
}