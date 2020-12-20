#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<pair<int64_t, int64_t>> ab(m);
    for (auto i : irange(0L, m)) {
        cin >> ab[i].first >> ab[i].second;
    }

    sort(ab.begin(), ab.end());

    int64_t ans = 0;
    int64_t p = n;
    for (const auto& t : ab | reversed) {
        if (p < t.second) {
            continue;
        }

        ++ans;
        p = t.first;
    }

    cout << ans << endl;
}