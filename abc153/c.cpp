#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> h(n);
    for (auto&& hh : h) {
        cin >> hh;
    }

    sort(h.begin(), h.end());

    int64_t ans = 0;
    for (auto i : irange(0L, max(n - k, 0L))) {
        ans += h[i];
    }

    cout << ans << endl;
}