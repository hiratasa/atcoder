#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> p(n);
    for (auto&& pp : p) {
        // b, a
        cin >> pp.second >> pp.first;
    }

    sort(p.begin(), p.end());

    int64_t t = 0;
    for (const auto& pp : p) {
        t += pp.second;
        if (t > pp.first) {
            cout << "No" << endl;
            return 0;
        }
    }

    cout << "Yes" << endl;
}