#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> p(n);
    for (auto&& pp : p) {
        cin >> pp.first >> pp.second;
    }

    double s = 0;
    for (auto pp : p) {
        for (auto qq : p) {
            s += sqrt((pp.first - qq.first) * (pp.first - qq.first) +
                      (pp.second - qq.second) * (pp.second - qq.second));
        }
    }

    cout << setprecision(10) << s / (double)n << endl;
}