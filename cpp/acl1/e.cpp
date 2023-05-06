#include <bits/stdc++.h>

#include <atcoder/lazysegtree>
#include <atcoder/modint>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace atcoder;
using namespace std;
using namespace boost;
using namespace boost::adaptors;

using Mod = modint998244353;

Mod op(Mod lhs, Mod rhs) { return lhs + rhs; }

Mod e() { return Mod(0); }

Mod mapping(Mod m, Mod v) { return m * v; }

Mod composition(Mod l, Mod r) { return l * r; }

Mod id() { return Mod(1); }

using ST = lazy_segtree<Mod, op, e, Mod, mapping, composition, id>;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector p(n, 0L);
    for (auto&& pp : p) {
        cin >> pp;
    }

    ST st(n + 1), st1(n + 1);

    Mod half = Mod(1) / 2;
    Mod m = Mod(1) - Mod(1) / k;

    Mod ans = 0;
    for (auto i : irange(0L, k)) {
        ans += st.prod(0, p[i]);
        ans += st1.prod(p[i], n + 1);
        ans -= st.prod(p[i], n + 1);

        st.set(p[i], half);
        st1.set(p[i], Mod(1));
    }

    for (auto i : irange(k, n)) {
        st.apply(0, n + 1, m);

        ans += st.prod(0, p[i]);
        ans += st1.prod(p[i], n + 1);
        ans -= st.prod(p[i], n + 1);

        st.set(p[i], half);
        st1.set(p[i], Mod(1));
    }

    cout << ans.val() << endl;
}