#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector l(n, 0L);
    for (auto&& ll : l) {
        cin >> ll;
    }

    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        for (auto j : irange(i + 1, n)) {
            for (auto k : irange(j + 1, n)) {
                auto mx = max({l[i], l[j], l[k]});
                auto mi = min({l[i], l[j], l[k]});
                auto me = l[i] + l[j] + l[k] - mx - mi;

                if (mx == me || me == mi) {
                    continue;
                }

                if (mx < me + mi) {
                    ++ans;
                }
            }
        }
    }

    cout << ans << endl;
}