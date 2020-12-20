#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<bitset<30>> a(n), b(n);
    bitset<30> x;
    for (auto i : irange(0L, n)) {
        int64_t aa;
        cin >> aa;
        a[i] = aa;
        b[i] = ((aa - 1) ^ aa);
        x ^= aa;
    }

    int64_t ans = 0;
    for (auto i : irange(0L, 30L) | reversed) {
        bitset<30> t;
        for (auto j : irange(0L, n)) {
            if (b[j][i]) {
                t = b[j];
                break;
            }
        }

        if (t.none()) {
            continue;
        }

        if (x[i]) {
            ++ans;
            x ^= t;
        }

        for (auto j : irange(0L, n)) {
            if (b[j][i]) {
                b[j] ^= t;
            }
        }
    }

    if (!x.none()) {
        ans = -1;
    }

    cout << ans << endl;
}