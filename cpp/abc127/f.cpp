#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t q;
    cin >> q;

    std::multiset<int64_t> a;
    auto it = a.begin();
    int64_t f = 0;
    for (auto _ : irange(0L, q)) {
        int o;
        cin >> o;

        if (o == 1) {
            // update
            int64_t aa, bb;
            cin >> aa >> bb;

            a.insert(aa);
            if (a.size() == 1) {
                it = a.begin();
            } else if (a.size() % 2 == 0) {
                f += abs(*it - aa);
                if (aa < *it) {
                    --it;
                }
            } else {
                f += abs(*it - aa);
                if (*it <= aa) {
                    auto prev = *it;
                    ++it;
                    f -= (*it - prev);
                }
            }

            f += bb;
        } else {
            // query
            cout << *it << " " << f << "\n";
        }
    }

    cout << flush;
}