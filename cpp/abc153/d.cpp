#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h;
    cin >> h;

    int64_t ans = 0;
    int64_t n = 1;
    while (h > 0) {
        if (h == 1) {
            ans += n;
            h = 0;
        } else {
            ans += n;
            h /= 2;
            n *= 2;
        }
    }

    cout << ans << endl;
}