#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    double b = 0.0;
    for (auto aa : a) {
        b += 1.0 / aa;
    }

    cout << setprecision(6) << 1 / b << endl;
}