#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    int64_t n4 = 0, n2 = 0;
    for (auto i : irange(0L, n)) {
        int64_t a;
        cin >> a;

        if (a % 4 == 0) {
            ++n4;
        } else if (a % 2 == 0) {
            ++n2;
        }
    }

    if ((n2 == 0 && n / 2 <= n4) || ((n - n2) + 1) / 2 <= n4) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}