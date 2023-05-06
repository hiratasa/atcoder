#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> a;
    int64_t s = 0;
    for (auto i : irange(0L, n)) {
        int64_t aa;
        cin >> aa;
        if (aa < k) {
            a.push_back(aa);
            s += aa;
        }
    }

    sort(a.rbegin(), a.rend());

    bitset<5000> bs;
    bs[0] = true;
    int64_t ans = 0;
    for (auto aa : a) {
        auto r = irange(max(k - s, 0L), k);
        bool ok = any_of(r.begin(), r.end(), [&](int64_t i) { return bs[i]; });
        if (!ok) {
            ++ans;
        }

        bs |= (bs << aa);
        s -= aa;
    }

    cout << ans << endl;
}