#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector<bool> b(n);
    for (auto _ : irange(0L, k)) {
        int64_t d;
        cin >> d;

        for (auto __ : irange(0L, d)) {
            int64_t a;
            cin >> a;
            b[a - 1] = true;
        }
    }

    int64_t ans = count(b.begin(), b.end(), false);
    cout << ans << endl;
}