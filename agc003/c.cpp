#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> a(n);
    for (auto i : irange(0L, n)) {
        cin >> a[i].first;
        a[i].second = i;
    }

    sort(a.begin(), a.end());

    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        if (i % 2 != a[i].second % 2) {
            ++ans;
        }
    }

    cout << ans / 2 << endl;
}