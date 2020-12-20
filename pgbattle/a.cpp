#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int n, c;
    cin >> n >> c;

    int worp = 0;
    for (auto i : irange(0, c)) {
        int l, r;
        cin >> l >> r;
        worp = max(worp, r - l);
    }

    cout << n - worp << endl;
}