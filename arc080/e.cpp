#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

class ST {
   public:
    using value_type = pair<int64_t, int64_t>;

    static constexpr auto K = 1L << 17;
    static constexpr auto M = 2 * K;
    static constexpr auto I = value_type{1L << 20, 1L << 20};

    ST() : values_(M, I), delay_swap_(M) {}

    void set(int64_t idx, const value_type& val) {
        idx += K - 1;

        values_[idx] = val;
        while (idx > 0) {
            idx = (idx - 1) / 2;
            values_[idx] = op(get(idx * 2 + 1), get(idx * 2 + 2));
        }
    }

    void set(int64_t idx, int64_t sub_idx, int64_t val) {
        if (sub_idx == 0) {
            set(idx, value_type{val, values_[idx + K - 1].second});
        } else {
            set(idx, value_type{values_[idx + K - 1].first, val});
        }
    }

    void swap_range(int64_t b, int64_t e, int64_t idx = 0, int64_t l = 0,
                    int64_t r = K) {
        if (e <= l || r <= b) {
            return;
        }

        if (b <= l && r <= e) {
            delay_swap_[idx] = !delay_swap_[idx];
            return;
        }

        swap_range(b, e, idx * 2 + 1, l, (l + r) / 2);
        swap_range(b, e, idx * 2 + 2, (l + r) / 2, r);

        values_[idx] = op(get(idx * 2 + 1), get(idx * 2 + 2));
    }

    value_type query(int64_t b, int64_t e, int64_t idx = 0, int64_t l = 0,
                     int64_t r = K) {
        if (e <= l || r <= b) {
            return I;
        }

        if (b <= l && r <= e) {
            return get(idx);
        }

        if (delay_swap_[idx]) {
            // propagate to children
            values_[idx] = get(idx);
            delay_swap_[idx * 2 + 1] = !delay_swap_[idx * 2 + 1];
            delay_swap_[idx * 2 + 2] = !delay_swap_[idx * 2 + 2];
            delay_swap_[idx] = false;
        }

        auto lhs = query(b, e, idx * 2 + 1, l, (l + r) / 2);
        auto rhs = query(b, e, idx * 2 + 2, (l + r) / 2, r);

        return op(lhs, rhs);
    }

   private:
    static value_type op(const value_type& l, const value_type& r) {
        return value_type{min(l.first, r.first), min(l.second, r.second)};
    }

    value_type get(int64_t idx) const {
        auto t = values_[idx];
        if (delay_swap_[idx]) {
            swap(t.first, t.second);
        }

        return t;
    }

    vector<value_type> values_;
    vector<bool> delay_swap_;
};

/* static */ constexpr ST::value_type ST::I;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> p(n), r(n);
    for (auto i : irange(0L, n)) {
        cin >> p[i];
        r[p[i]] = i;
    }

    auto m = n / 2;

    ST st;
    for (auto i : irange(0L, m)) {
        st.set(i, {p[2 * i], p[2 * i + 1]});
    }

    const auto* delim = "";
    set<int64_t> picked{n};
    for (auto i : irange(0L, m)) {
        auto a = st.query(0L, m).first;
        auto ia = r[a];

        auto bound = *picked.upper_bound(ia);
        auto b = st.query((ia + 1) / 2, (bound + 1) / 2).second;

        cout << delim << a << " " << b;
        delim = " ";

        auto ib = r[b];

        st.set(ia / 2, ia % 2, 1L << 20);
        st.set(ib / 2, ib % 2, 1L << 20);
        st.swap_range((ia + 1) / 2, (ib + 1) / 2);
        picked.insert(ia);
        picked.insert(ib);
    }

    cout << endl;
}