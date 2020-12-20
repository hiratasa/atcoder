#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int n;
    cin >> n;
    vector<int> a(n);
    int s = 0;
    for (auto&& aa : a) {
        cin >> aa;
        s += aa;
    }

    int min = numeric_limits<int>::max();
    int min_index = -1;
    for (auto i : irange(0, n)) {
        auto aa = a[i];
        if (abs(aa * n - s) < min) {
            min = abs(aa * n - s);
            min_index = i;
        }
    }

    cout << min_index << endl;
}