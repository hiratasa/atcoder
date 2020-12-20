#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> x(n);
    int64_t s;
    for (auto&& xx : x) {
        cin >> xx;
        s += xx;
    }

    int64_t p1 = s / n, p2 = p1 + 1;
    int64_t k1 = 0, k2 = 0;
    for (auto&& xx : x) {
        k1 += (xx - p1) * (xx - p1);
        k2 += (xx - p2) * (xx - p2);
    }

    cout << min(k1, k2) << endl;
}