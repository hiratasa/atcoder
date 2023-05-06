#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    bitset<100000> bs;
    cin >> bs;

    vector<int64_t> p;
    p.push_back(0L);
    if (!bs[0]) {
        p.push_back(0L);
    }
    for (auto i : irange(1L, n)) {
        if (bs[i] != bs[i - 1]) {
            p.push_back(i);
        }
    }
    p.push_back(n);

    int64_t ans = 0;
    for (auto i : irange(0uL, p.size())) {
        if (i % 2 == 0) {
            ans = max(ans, p[min(p.size() - 1, i + 2 * k + 1)] - p[i]);
        }
    }
    cout << ans << endl;
}