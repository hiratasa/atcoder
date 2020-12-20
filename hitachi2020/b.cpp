#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t A, B, M;
    cin >> A >> B >> M;

    vector<int64_t> a(A);
    int64_t ma = numeric_limits<int64_t>::max();
    for (auto&& aa : a) {
        cin >> aa;
        ma = min(aa, ma);
    }

    vector<int64_t> b(B);
    int64_t mb = numeric_limits<int64_t>::max();
    for (auto&& bb : b) {
        cin >> bb;
        mb = min(bb, mb);
    }

    int64_t ans = ma + mb;
    for (auto i : irange(0L, M)) {
        int64_t x, y, c;
        cin >> x >> y >> c;

        ans = min(ans, a[x - 1] + b[y - 1] - c);
    }

    cout << ans << endl;
}