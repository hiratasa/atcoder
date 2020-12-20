#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int n;
    cin >> n;

    vector<int> xs(n), ys(n), rs(n);
    for (auto i : irange(0, n)) {
        cin >> xs[i] >> ys[i] >> rs[i];
    }

    vector<int> counts(n + 1);
    for (int x = -60; x <= 60; ++x) {
        for (int y = -60; y <= 60; ++y) {
            int c = 0;
            for (auto i : irange(0, n)) {
                if ((x - xs[i]) * (x- xs[i]) + (y - ys[i]) * (y - ys[i]) <= rs[i] * rs[i]) {
                    ++c;
                }
            }
            counts[c]++;
        }
    }

    for (auto c : irange(1, n + 1)) {
        cout << counts[c] << endl;
    }
}