#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

class BIT {
    static constexpr auto C = 1L << 18;
    static constexpr auto I = -1L;

   public:
    BIT() : values_(C + 1, I) {}

    void set(int64_t idx, int64_t value) {
        while (idx <= C) {
            values_[idx] = max(values_[idx], value);
            idx += (idx & -idx);
        }
    }

    // [1, idx]
    int64_t query(int64_t idx) {
        int64_t value = I;
        while (idx > 0) {
            value = max(values_[idx], value);
            idx -= (idx & -idx);
        }
        return value;
    }

   private:
    vector<int64_t> values_;
};

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> l(n);
    BIT bit;
    for (auto i : irange(0L, n)) {
        l[i] = bit.query(a[i]);
        bit.set(a[i], i);
    }

    vector<int64_t> r(n);
    bit = BIT();
    for (auto i : irange(0L, n) | reversed) {
        r[i] = n - 1 - bit.query(a[i]);
        bit.set(a[i], n - 1 - i);
    }

    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        // cerr << l[i] << " " << r[i] << endl;
        ans += a[i] * (r[i] - i) * (i - l[i]);
    }

    cout << ans << endl;
}