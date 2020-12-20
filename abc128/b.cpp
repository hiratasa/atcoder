#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<pair<pair<string, int64_t>, int64_t>> s(n);
    for (auto i : irange(0L, n)) {
        cin >> s[i].first.first >> s[i].first.second;
        s[i].first.second *= -1;
        s[i].second = i + 1;
    }

    sort(s.begin(), s.end());

    for (const auto& ss : s) {
        cout << ss.second << endl;
    }
}