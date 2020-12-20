#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    sort(a.begin(), a.end());

    vector<pair<int64_t, int64_t>> p(m);
    for (auto&& pp : p) {
        cin >> pp.second >> pp.first;
    }

    sort(p.begin(), p.end());

    int64_t s = 0;
    for (auto aa : a) {
        if (!p.empty() && aa < p.back().first) {
            s += p.back().first;
            --p.back().second;
            if (p.back().second == 0) {
                p.pop_back();
            }
        } else {
            s += aa;
        }
    }

    cout << s << endl;
}