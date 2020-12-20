#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t input() {
    int64_t x;
    cin >> x;
    return x;
}

int main() {
    vector<pair<int64_t, int64_t>> c;
    c.emplace_back(1, input());
    c.emplace_back(5, input());
    c.emplace_back(10, input());
    c.emplace_back(50, input());
    c.emplace_back(100, input());
    c.emplace_back(500, input());

    auto a = input();

    int64_t ans = 0;
    for (auto [v, cc] : c | reversed) {
        auto nn = min(a / v, cc);
        ans += nn;
        a -= v * nn;
    }

    cout << ans << endl;
}