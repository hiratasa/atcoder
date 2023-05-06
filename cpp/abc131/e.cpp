#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, k;
    cin >> n >> k;

    auto m = n * (n - 1) / 2 - k;
    if (m < n - 1) {
        cout << "-1" << endl;
        return 0;
    }
    cout << m << endl;

    for (auto i : irange(1L, n + 1)) {
        for (auto j : irange(i + 1, n + 1)) {
            cout << i << " " << j << "\n";
            --m;

            if (m == 0) {
                return 0;
            }
        }
    }

}