#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, a, b;
    cin >> n >> a >> b;

    vector<int64_t> x(n);
    for (auto i : irange(0L, n)) {
        cin >> x[i];
    }

    int64_t ans = 0;
    for (auto i : irange(1L, n)) {
        ans += min((x[i] - x[i - 1]) * a, b);
    }

    cout << ans << endl;
}