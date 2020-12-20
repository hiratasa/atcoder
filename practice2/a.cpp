#include <bits/stdc++.h>

#include <atcoder/dsu>
#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, q;
    cin >> n >> q;

    atcoder::dsu d(n);

    for (auto _ : irange(0L, q)) {
        int64_t t, u, v;
        cin >> t >> u >> v;

        if (t == 0) {
            d.merge(u, v);
        } else {
            cout << (int)d.same(u, v) << endl;
        }
    }
}