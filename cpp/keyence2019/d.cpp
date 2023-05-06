#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

constexpr int64_t M = 1000000007L;

main() {
    int64_t n, m;
    cin >> n >> m;
    
    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> b(m);
    for (auto&& bb : b) {
        cin >> bb;
    }

    sort(a.begin(), a.end());
    sort(b.begin(), b.end());
    
    for (auto i : irange(1L, n)) {
        if (a[i - 1] == a[i]) {
            cout << "0" << endl;
            return 0;
        }
    }

    for (auto i : irange(1L, m)) {
        if (b[i - 1] == b[i]) {
            cout << "0" << endl;
            return 0;
        }
    }

    int64_t ans = 1;
    for (auto i : irange(1L, n * m + 1)) {
        auto ita = partition_point(a.begin(), a.end(), [i](int64_t aa) {
            return aa < i;
        });

        if (ita == a.end()) {
            cout << "0" << endl;
            return 0;
        }

        auto itb = partition_point(b.begin(), b.end(), [i](int64_t bb) {
            return bb < i;
        });

        if (itb == b.end()) {
            cout << "0" << endl;
            return 0;
        }

        auto va = *ita;
        auto vb = *itb;

        auto rem = (a.end() - ita) * (b.end() - itb) - (n * m - i);
        if (rem <= 0) {
            cout << "0" << endl;
            return 0;
        }

        if (va == i && vb == i) {
            // NOP
        } else if (va == i) {
            ans = (ans * (b.end() - itb)) % M;
        } else if (vb == i) {
            ans = (ans * (a.end() - ita)) % M;
        } else {
            ans = ans * ((a.end() - ita) * (b.end() - itb) - (n * m - i)) % M;
        }
    }

    cout << ans << endl;
}