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

    vector<pair<int64_t, int64_t>> pos;
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;
        pos.emplace_back(a, 1);
    }
    for (auto i : irange(0L, n)) {
        int64_t b;
        cin >> b;
        pos.emplace_back(b, -1);
    }

    sort(pos.begin(), pos.end());

    constexpr auto M = 1000000007L;

    int64_t ans = 1, c = 0;
    for (const auto& p : pos) {
        if (c * p.second < 0) {
            ans *= abs(c);
            ans %= M;
        }
        c += p.second;
    }

    cout << ans << endl;
}