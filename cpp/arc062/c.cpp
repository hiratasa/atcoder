#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<array<int64_t, 2>> ta(n);
    for (auto&& x : ta) {
        cin >> x[0] >> x[1];
    }

    array<int64_t, 2> ans = ta[0];
    for (const auto& x : ta) {
        auto y = max((ans[0] - 1) / x[0] + 1, (ans[1] - 1) / x[1] + 1);
        ans[0] = x[0] * y;
        ans[1] = x[1] * y;
    }

    cout << ans[0] + ans[1] << endl;
}