#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    int64_t mi = n, ma = 0;
    for (auto&& aa : a) {
        cin >> aa;
        mi = min(mi, aa);
        ma = max(ma, aa);
    }

    if (ma - mi > 1) {
        cout << "No" << endl;
    } else if (ma == mi + 1) {
        auto k = count(a.begin(), a.end(), mi);
        auto l = k + 1;
        auto u = k + (n - k) / 2;

        if (l <= ma && ma <= u) {
            cout << "Yes" << endl;
        } else {
            cout << "No" << endl;
        }
    } else {
        if (mi == n - 1) {
            cout << "Yes" << endl;
        } else if (1 <= ma && ma <= n / 2) {
            cout << "Yes" << endl;
        } else {
            cout << "No" << endl;
        }
    }
}