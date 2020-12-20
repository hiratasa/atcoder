#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    int64_t ans = 1;
    while (k <= n) {
        ++ans;
        n /= k;
    }

    cout << ans << endl;
}