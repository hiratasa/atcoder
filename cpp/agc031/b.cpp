#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main(){
    constexpr auto M = 1000000007;

    int64_t n;
    cin >> n;

    vector<int64_t> c(n);
    for (auto&& cc : c) {
        cin >> cc;
    }

    vector<int64_t> last(200001L, -1);
    vector<int64_t> counts(n + 1);
    counts[0] = 1;
    for (auto i : irange(0L, n)) {
        auto cc = c[i];

        counts[i + 1] = counts[i];
        auto ll = last[cc];
        if (ll >= 0 && ll + 1 < i) {
            counts[i + 1] += counts[ll + 1];
            counts[i + 1] %= M;
        }

        last[cc] = i;
    }

    cout << counts.back() << endl;
}