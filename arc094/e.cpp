#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n), b(n);
    int64_t m = numeric_limits<int64_t>::max(), s = 0;
    for (auto i : irange(0L, n)) {
        cin >> a[i] >> b[i];
        s += a[i];
        if (a[i] > b[i]) {
            m = min(m, b[i]);
        }
    }

    if (m == numeric_limits<int64_t>::max()) {
        cout << 0 << endl;
        return 0;
    }

    cout << s - m << endl;
}