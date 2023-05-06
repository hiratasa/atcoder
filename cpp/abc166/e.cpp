#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    unordered_map<int64_t, int64_t> mp, mm;
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;

        ++mp[a + i];
        ++mm[a - i];
    }

    int64_t ans = 0;
    for (const auto& [t, m] : mp) {
        ans += m * mm[-t];
    }

    cout << ans << endl;
}