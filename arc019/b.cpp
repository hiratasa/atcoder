#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string a;
    cin >> a;

    int64_t k = 0;
    for (auto i : irange(0uL, a.size() / 2)) {
        if (a[i] != a[a.size() - i - 1]) {
            ++k;
        }
    }

    if (k == 0) {
        cout << ('Z' - 'A') * (a.size() / 2 * 2) << endl;
    } else if (k == 1) {
        cout << ('Z' - 'A') * a.size() - 2 << endl;
    } else {
        cout << ('Z' - 'A') * a.size() << endl;
    }
}