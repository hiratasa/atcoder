#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t x, n;
    cin >> x >> n;

    unordered_set<int64_t> p;
    for (auto _ : irange(0L, n)) {
        int64_t pp;
        cin >> pp;
        p.insert(pp);
    }

    int64_t ans = 0;
    for (auto i : irange(1L, 102L)) {
        if (p.count(i)) {
            continue;
        }
        if (abs(x - i) < abs(x - ans)) {
            ans = i;
        }
    }

    cout << ans << endl;
}