#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    int64_t r, s, p;
    cin >> r >> s >> p;

    string t;
    cin >> t;

    int64_t ans = 0;
    unordered_map<char, int64_t> m;
    m['r'] = p;
    m['s'] = r;
    m['p'] = s;
    for (auto i : irange(0L, k)) {
        char prev = 0;
        for (int64_t j = i; j < n; j += k) {
            if (prev != t[j]) {
                ans += m[t[j]];
                prev = t[j];
            } else {
                prev = 0;
            }
        }
    }

    cout << ans << endl;
}