#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> ans(n);
    for (auto i : irange(0L, n - 1)) {
        int64_t a;
        cin >> a;
        ++ans[a - 1];
    }

    for (auto i : irange(0L, n)) {
        cout << ans[i] << "\n";
    }
}