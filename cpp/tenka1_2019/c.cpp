#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    vector<int64_t> w;
    w.push_back(0);
    for (auto c : s) {
        w.push_back(w.back());
        if (c == '.') {
            ++w.back();
        }
    }

    int64_t m = numeric_limits<int64_t>::max();
    for (auto i : irange(0L, n + 1)) {
        auto mm = (i - w[i]) + (w.back() - w[i]);
        m = min(m, mm);
    }

    cout << m << endl;
}