#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

class BIT {
   public:
    explicit BIT(int64_t n) : values(n + 1) {}

    void inc(int64_t idx) {
        ++idx;

        while (idx < values.size()) {
            ++values[idx];
            idx += (idx & -idx);
        }
    }

    // [0, idx) in 0-indexed
    // [1, idx] in 1-indexed
    int64_t query(int64_t idx) {
        int64_t ret = 0;

        while (idx > 0) {
            ret += values[idx];
            idx -= (idx & -idx);
        }

        return ret;
    }

    vector<int64_t> values;
};

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> b(n);
    map<int64_t, int64_t> idxs;
    int64_t s = 0;
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;

        s += a;
        b[i] = s - k * (i + 1);
        idxs[b[i]] = 0;
    }

    idxs[0] = 0;

    int64_t idx = 0;
    for (auto& kv : idxs) {
        kv.second = idx++;
    }

    BIT bit(idxs.size());
    bit.inc(idxs[0]);
    int64_t ans = 0;
    for (auto bb : b) {
        ans += bit.query(idxs[bb] + 1);
        bit.inc(idxs[bb]);
    }

    cout << ans << endl;
}