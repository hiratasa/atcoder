#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto i : irange(0L, n)) {
        cin >> a[i];
    }

    sort(a.begin(), a.end());

    vector<int64_t> s(n + 1);
    for (auto i : irange(0L, n)) {
        s[i + 1] = s[i] + a[i];
    }

    int64_t ans = 1;
    for (auto i : irange(0L, n - 1) | reversed) {
        if (2 * s[i + 1] < a[i + 1]) {
            break;
        }

        ++ans;
    }

    cout << ans << endl;
}