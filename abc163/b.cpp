#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    int64_t s = 0;
    for (auto i : irange(0L, m)) {
        int64_t a;
        cin >> a;
        s += a;
    }

    if (s > n) {
        cout << -1 << endl;
    } else {
        cout << n - s << endl;
    }
}