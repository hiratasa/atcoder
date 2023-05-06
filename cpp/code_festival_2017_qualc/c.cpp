#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    string t;
    vector<int64_t> nx;
    int64_t m = 0;
    for (auto c : s) {
        if (c == 'x') {
            ++m;
        } else {
            t.push_back(c);
            nx.push_back(m);
            m = 0;
        }
    }
    nx.push_back(m);

    auto tr = t;
    reverse(tr.begin(), tr.end());
    if (t != tr) {
        cout << -1 << endl;
        return 0;
    }

    int64_t ans = 0;
    for (auto i : irange(0uL, nx.size() / 2)) {
        ans += abs(nx[i] - nx[nx.size() - 1 - i]);
    }

    cout << ans << endl;
}