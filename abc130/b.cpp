#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, x;
    cin >> n >> x;

    int64_t count = 1;
    int64_t pos = 0;
    for (auto _ : irange(0L, n)) {
        int64_t l;
        cin >> l;

        pos += l;
        if (pos > x) {
            cout << count << endl;
            return 0;
        }

        ++count;
    }

    cout << count << endl;
    return 0;
}