#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<string> s(n);
    for (auto&& ss : s) {
        cin >> ss;
    }

    int64_t ans = 0;
    for (auto d : irange(0L, n)) {
        bool ok = true;
        for (auto i : irange(0L, n)) {
            for (auto j : irange(0L, n)) {
                if (s[(i + d) % n][j] != s[(j + d) % n][i]) {
                    ok = false;
                    break;
                }
            }
            if (!ok) {
                break;
            }
        }

        if (ok) {
            ans += n;
        }
    }

    cout << ans << endl;
}