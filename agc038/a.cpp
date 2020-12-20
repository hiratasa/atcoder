#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t h, w, a, b;
    cin >> h >> w >> a >> b;

    for (auto i : irange(0L, h)) {
        for (auto j : irange(0L, a)) {
            if (i < b) {
                cout << 0;
            } else {
                cout << 1;
            }
        }
        for (auto j : irange(a, w)) {
            if (i < b) {
                cout << 1;
            } else {
                cout << 0;
            }
        }

        cout << "\n";
    }
}