#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    for (int64_t c = 3, i = 1; c < n; c *= 3, ++i) {
        auto m = n - c;

        for (int64_t d = 5, j = 1; d <= m; d *= 5, ++j) {
            if (c + d == n) {
                cout << i << " " << j << endl;
                return 0;
            }
        }
    }

    cout << -1 << endl;
}