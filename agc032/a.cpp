#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> b(n);
    for (auto&& bb : b) {
        cin >> bb;
    }

    vector<int64_t> ops;
    for (auto i : irange(0L, n)) {
        int64_t j = n - i;
        for (auto it = b.rbegin(); it != b.rend(); ++it) {
            auto&& bb = *it;
            if (bb == -1) {
                continue;
            }

            if (j == bb) {
                ops.push_back(bb);
                bb = -1;
                break;
            }
            --j;
        }

        if (ops.size() < i + 1) {
            cout << "-1" << endl;
            return 0;
        }
    }

    for (auto it = ops.rbegin(); it != ops.rend(); ++it) {
        cout << *it << endl;
    }
}