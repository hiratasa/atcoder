#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

class ST {
    static constexpr auto C = 1L << 18;
    static constexpr auto S = 2 * C;
    static constexpr auto I = -1L;

   public:
    ST() : values_(S, I) {}

    void set(int64_t idx, int64_t value) {
        idx += C - 1;

        values_[idx] = value;
        while (idx > 0) {
            idx = (idx - 1) / 2;
            values_[idx] = max(values_[2 * idx + 1], values_[2 * idx + 2]);
        }
    }

    int64_t query(int64_t b, int64_t e) { return query(b, e, 0L, 0L, C); }

    int64_t query(int64_t b, int64_t e, int64_t idx, int64_t l,
                  int64_t r) const {
        if (e <= l || r <= b) {
            return I;
        }

        if (b <= l && r <= e) {
            return values_[idx];
        }

        return max(query(b, e, 2 * idx + 1, l, (l + r) / 2),
                   query(b, e, 2 * idx + 2, (l + r) / 2, r));
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
    ST st;
    for (auto i : irange(0L, n)) {
        l[i] = st.query(0L, a[i]);
        st.set(a[i], i);
    }

    vector<int64_t> r(n);
    st = ST();
    for (auto i : irange(0L, n) | reversed) {
        r[i] = n - 1 - st.query(0L, a[i]);
        st.set(a[i], n - 1 - i);
    }

    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        cerr << l[i] << " " << r[i] << endl;
        ans += a[i] * (r[i] - i) * (i - l[i]);
    }

    cout << ans << endl;
}