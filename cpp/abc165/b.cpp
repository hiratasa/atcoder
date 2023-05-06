#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    int64_t m = 100;
    int64_t ans = 0;
    while (m < n) {
        m = m * 1.01;
        ++ans;
    }

    cout << ans << endl;
}