#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
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
    }

    for (auto u : irange(0uL, 1uL << n)) {
        bitset<20> bs(u);

        int64_t s = 0;
        for (auto i : irange(0L, n)) {
            s += bs[i] * a[i];
        }

        if (s == k) {
            cout << "Yes" << endl;
            return 0;
        }
    }

    cout << "No" << endl;
}