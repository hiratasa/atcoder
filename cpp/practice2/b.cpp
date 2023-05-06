#include <bits/stdc++.h>

#include <atcoder/fenwicktree>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace atcoder;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, q;
    cin >> n >> q;

    fenwick_tree<int64_t> ft(n);

    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;
        ft.add(i, a);
    }

    for (auto _ : irange(0L, q)) {
        int64_t t, a, b;
        cin >> t >> a >> b;

        if (t == 0) {
            ft.add(a, b);
        } else {
            cout << ft.sum(a, b) << endl;
        }
    }
}