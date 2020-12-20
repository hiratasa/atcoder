#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b, c, k;
    cin >> a >> b >> c >> k;

    while (b <= a) {
        --k;
        b *= 2;
    }

    while (c <= b) {
        --k;
        c *= 2;
    }

    cout << (k >= 0 ? "Yes" : "No") << endl;
}