#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    int64_t k = 1;
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;

        if (a == k) {
            ++k;
        }
    }

    if (k == 1) {
        cout << -1 << endl;
    } else {
        cout << n - (k - 1) << endl;
    }
}