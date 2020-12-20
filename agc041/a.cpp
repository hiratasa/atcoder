#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, a, b;
    cin >> n >> a >> b;

    if ((b - a) % 2 == 0) {
        cout << (b - a) / 2 << endl;
    } else {
        int64_t d1 = a + ((b - a) - 1) / 2;
        int64_t d2 = (n - b + 1) + (n - (a + n - b + 1)) / 2;

        cout << min(d1, d2) << endl;
    }
}