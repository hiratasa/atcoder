#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

struct Mod {
    static constexpr auto kMod = 998244353L;

    Mod() : n(0) {}
    // can be implicitly converted
    Mod(int64_t n) : n(n) {}

    Mod operator*(Mod m) const { return (n * (m.n % kMod)) % kMod; }

    Mod& operator*=(Mod m) {
        *this = *this * m;
        return *this;
    }

    Mod pow(int64_t p) {
        if (p == 0) {
            return 1;
        }
        if (p == 1) {
            return n;
        }

        int64_t r = this->pow(p / 2).n;
        if (p % 2 == 0) {
            return r * r % kMod;
        } else {
            return (r * r % kMod) * n % kMod;
        }
    }

    Mod operator/(Mod m) const {
        if (n == 0) {
            return 0;
        }

        if (m.n == 0) {
            throw;
        }

        return *this * m.pow(kMod - 2);
    }

    Mod& operator/=(Mod m) {
        *this = *this / m;
        return *this;
    }

    Mod operator+(Mod m) const { return (n + m.n) % kMod; }

    Mod& operator+=(Mod m) {
        *this = *this + m;
        return *this;
    }

    Mod operator-(Mod m) const { return (kMod + n - m.n) % kMod; }

    Mod& operator-=(Mod m) {
        *this = *this - m;
        return *this;
    }

    int64_t n;
};

// M (type, id, op): Monoid of value
// Op (type, id, op, apply): Monoid of lazy operation
template <typename M, typename Op, typename F>
class SegmentTree {
   public:
    using value_type = typename M::type;
    using operator_type = typename Op::type;

    explicit SegmentTree(int64_t n)
            : f(F{}),
              n(n),
              height(ceil(log2(n)) + 1),
              cap(pow(2, ceil(log2(n)))),
              values(2 * cap - 1, M::id()),
              lazy(2 * cap - 1, Op::id()) {}

    void set(int64_t pos, value_type v) {
        int64_t idx = cap - 1 + pos;

        for (auto i : irange(1L, height) | reversed) {
            auto parent_idx = ((idx + 1) >> i) - 1;

            auto left_idx = 2 * (parent_idx + 1) - 1;
            auto right_idx = 2 * (parent_idx + 1);

            lazy[left_idx] = Op::op(lazy[parent_idx], lazy[left_idx]);
            lazy[right_idx] = Op::op(lazy[parent_idx], lazy[right_idx]);
            lazy[parent_idx] = Op::id();

            fix_value(parent_idx);
        }

        values[idx] = v;
        lazy[idx] = Op::id();

        while (idx > 0) {
            idx = (idx - 1) / 2;
            fix_value(idx);
        }
    }

    void update(int64_t a, int64_t b, operator_type p) {
        update(a, b, p, 0, 0, cap);
    }

    void update(int64_t a, int64_t b, operator_type p, int64_t idx, int64_t l,
                int64_t r) {
        if (a >= r || b <= l) {
            // no overlap
            return;
        }

        if (a <= l && r <= b) {
            lazy[idx] = Op::op(p, lazy[idx]);
            return;
        }

        auto left_idx = 2 * (idx + 1) - 1;
        auto right_idx = 2 * (idx + 1);

        // モノイドOpが可換でない場合、pの適用前にlazy[idx]の適用が必要
        lazy[left_idx] = Op::op(lazy[idx], lazy[left_idx]);
        lazy[right_idx] = Op::op(lazy[idx], lazy[right_idx]);
        lazy[idx] = Op::id();

        update(a, b, p, left_idx, l, (l + r) / 2);
        update(a, b, p, right_idx, (l + r) / 2, r);

        fix_value(idx);
    }

    value_type query(int64_t a, int64_t b) { return query(a, b, 0, 0, cap); }

    value_type query(int64_t a, int64_t b, int64_t idx, int64_t l, int64_t r) {
        if (a >= r || b <= l) {
            // no overlap
            return M::id();
        }

        if (a <= l && r <= b) {
            return get_node_value(idx);
        }

        auto left_idx = 2 * (idx + 1) - 1;
        auto right_idx = 2 * (idx + 1);

        auto left_v = query(a, b, left_idx, l, (l + r) / 2);
        auto right_v = query(a, b, right_idx, (l + r) / 2, r);

        return f(lazy[idx], M::op(left_v, right_v));
    }

   private:
    value_type get_node_value(int64_t idx) { return f(lazy[idx], values[idx]); }

    void fix_value(int64_t idx) {
        auto left_idx = 2 * (idx + 1) - 1;
        auto right_idx = 2 * (idx + 1);
        values[idx] =
                M::op(get_node_value(left_idx), get_node_value(right_idx));
    }

    F f;
    int64_t n;
    int64_t height;
    int64_t cap;
    vector<value_type> values;
    vector<operator_type> lazy;
};

struct ModSumLen {
    using type = pair<Mod, int64_t>;

    static type id() { return make_pair(Mod(0L), 0L); }
    static type op(type v1, type v2) {
        return make_pair(v1.first + v2.first, v1.second + v2.second);
    }
};

struct Affine {
    using type = pair<Mod, Mod>;

    static type id() { return make_pair(Mod(1L), Mod(0L)); }
    static type op(type v1, type v2) {
        return make_pair(v1.first * v2.first, v1.first * v2.second + v1.second);
    }
};

struct AffineTransform {
    pair<Mod, int64_t> operator()(pair<Mod, Mod> x, pair<Mod, int64_t> y) {
        return make_pair(x.first * y.first + x.second * y.second, y.second);
    }
};

using ST = SegmentTree<ModSumLen, Affine, AffineTransform>;

int main() {
    int64_t n, q;
    cin >> n >> q;

    ST st(n);
    for (auto i : irange(0L, n)) {
        int64_t aa;
        cin >> aa;
        st.set(i, make_pair(aa, 1L));
    }

    for (auto _ : irange(0L, q)) {
        int64_t t;
        cin >> t;

        if (t == 0) {
            int64_t l, r, b, c;
            cin >> l >> r >> b >> c;

            st.update(l, r, make_pair(b, c));
        } else {
            int64_t l, r;
            cin >> l >> r;

            cout << st.query(l, r).first.n << "\n";
        }
    }
}