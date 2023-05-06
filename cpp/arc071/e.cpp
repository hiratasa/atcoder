#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s, t;
    cin >> s >> t;

    vector<int64_t> x(s.size() + 1), y(t.size() + 1);
    for (auto i : irange(0uL, s.size())) {
        x[i + 1] = x[i] + (s[i] == 'B' ? 2 : 1);
    }
    for (auto i : irange(0uL, t.size())) {
        y[i + 1] = y[i] + (t[i] == 'B' ? 2 : 1);
    }

    int64_t q;
    cin >> q;

    for (auto _ : irange(0L, q)) {
        int64_t a, b, c, d;
        cin >> a >> b >> c >> d;

        auto xx = x[b] - x[a - 1];
        auto yy = y[d] - y[c - 1];

        if ((xx - yy) % 3 == 0) {
            cout << "YES\n";
        } else {
            cout << "NO\n";
        }
    }
}