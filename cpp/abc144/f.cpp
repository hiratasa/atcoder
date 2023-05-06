#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n, m;
    cin >> n >> m;

    vector<vector<int64_t>> links(n);
    for (auto i : irange(0L, m)) {
        int64_t s, t;
        cin >> s >> t;
        --s;
        --t;
        links[s].push_back(t);
    }

    vector<double> e(n), e2(n);
    for (auto s : irange(0L, n - 1) | reversed) {
        double se = 0, me = 0;
        for (auto t : links[s]) {
            se += e[t];
            me = max(me, e[t]);
        }
        e[s] = se / links[s].size() + 1;
        if (links[s].size() > 1) {
            e2[s] = e[s] - ((se - me) / (links[s].size() - 1) + 1);
        }
    }

    double ans = e[0];
    vector<double> p(n);
    p[0] = 1;
    for (auto s : irange(0L, n)) {
        ans = min(ans, e[0] - p[s] * e2[s]);

        for (auto t : links[s]) {
            p[t] += p[s] / links[s].size();
        }
    }

    cout << setprecision(10) << ans << endl;
}