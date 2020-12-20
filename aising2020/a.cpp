#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t l, r, d;
    cin >> l >> r >> d;

    int64_t ans = 0;
    for (auto i : irange(l, r + 1)) {
        ans += (i % d == 0 ? 1 : 0);
    }

    cout << ans << endl;
}