#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector a(n, 0L);
    for (auto&& aa : a) {
        cin >> aa;
        --aa;
    }

    vector b{0L};
    unordered_map<int64_t, int64_t> um{{0L, 0L}};
    for (int64_t i = 0, c = 0;; ++i) {
        c = a[c];

        if (i + 1 == k) {
            cout << c + 1 << endl;
            return 0;
        }

        if (um.count(c)) {
            auto p = i + 1 - um[c];
            k = (k - um[c]) % p + um[c];
            cout << b[k] + 1 << endl;
            return 0;
        }

        b.push_back(c);
        um[c] = i + 1;
    }
}