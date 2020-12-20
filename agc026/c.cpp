#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    unordered_map<pair<string, string>, int64_t,
                  boost::hash<pair<string, string>>>
            m;
    for (auto u : irange(0uL, 1uL << n)) {
        bitset<18> bs(u);

        string s1, s2;
        for (auto i : irange(0L, n)) {
            if (bs[i]) {
                s1.push_back(s[i]);
            } else {
                s2.push_back(s[i]);
            }
        }

        ++m[make_pair(s1, s2)];
    }

    int64_t ans = 0;
    for (auto u : irange(0uL, 1uL << n)) {
        bitset<18> bs(u);

        string s1, s2;
        for (auto i : irange(n, 2 * n) | reversed) {
            if (bs[i - n]) {
                s1.push_back(s[i]);
            } else {
                s2.push_back(s[i]);
            }
        }

        ans += m[make_pair(s1, s2)];
    }

    cout << ans << endl;
}