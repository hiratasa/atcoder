#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector<int64_t> p(n);
    for (auto&& pp : p) {
        cin >> pp;
    }

    int64_t s = 0;
    for (auto i : irange(0L, k)) {
        s += p[i];
    }
    int64_t m = s;
    for (auto i : irange(1L, n - k + 1)) {
        s += -p[i - 1] + p[i + k - 1];
        m = max(m, s);
    }

    cout << setprecision(10) << (m + k) / 2.0 << endl;
}