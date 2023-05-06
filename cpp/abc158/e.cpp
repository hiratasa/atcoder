#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, p;
    cin >> n >> p;

    string s;
    cin >> s;

    vector<int64_t> v;
    for (auto c : s) {
        v.push_back(c - '0');
    }

    if (p == 2 || p == 5) {
        int64_t ans = 0;
        for (auto i : irange(0L, n)) {
            if (v[i] % p == 0) {
                ans += i + 1;
            }
        }

        cout << ans << endl;
        return 0;
    }

    int64_t ans = 0;
    int64_t m = 0, d = 1;
    unordered_map<int64_t, int64_t> u;
    u[0] = 1;
    for (auto i : irange(0L, n) | reversed) {
        m += v[i] * d;
        m %= p;
        d *= 10;
        d %= p;
        ans += u[m];
        ++u[m];
    }

    cout << ans << endl;
}