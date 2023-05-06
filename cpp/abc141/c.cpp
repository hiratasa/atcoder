#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, k, q;
    cin >> n >> k >> q;

    vector<int64_t> s(n + 1, 0);
    for (auto _ : irange(0L, q)) {
        int64_t a;
        cin >> a;

        ++s[a];
    }

    for (auto i : irange(1L, n + 1)) {
        auto score = k - q + s[i];
        if (score <= 0) {
            cout << "No\n";
        } else {
            cout << "Yes\n";
        }
    }
}