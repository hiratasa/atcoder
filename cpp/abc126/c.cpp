#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int n, k;
    cin >> n >> k;

    double ans = 0.0;
    int prev = n + 1;
    for (int x = k, i = 0; x > 0; x = (x + 1) / 2, ++i) {
        if (x < prev) {
            ans += (prev - x) / pow(2.0, i);
            prev = x;
        }

        if (x == 1) {
            break;
        }
    }

    cout << setprecision(16) << ans / n << endl;
}