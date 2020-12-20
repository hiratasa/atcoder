#include <bits/stdc++.h>
#include <boost/functional/hash.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    auto r = irange(0L, n / 2 + 1);
    auto it = partition_point(r.begin(), r.end(), [&](int64_t len) {
        if (len == 0) {
            return true;
        }

        unordered_map<string, int64_t, boost::hash<string>> m;
        for (auto p : irange(0L, n - len + 1)) {
            string sub = string(s.begin() + p, s.begin() + p + len);
            if (m.count(sub) > 0) {
                if (m[sub] + len <= p) {
                    cerr << sub << endl;
                    return true;
                }
            } else {
                m[sub] = p;
            }
        }

        return false;
    });

    cout << *it - 1 << endl;
}