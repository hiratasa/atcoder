#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    bool has_one = false;
    for (auto&& aa : a) {
        char c;
        cin >> c;
        aa = c - '0' - 1;

        if (aa == 1) {
            has_one = true;
        }
    }

    bool f = false;
    for (auto i : irange(0L, n)) {
        if (has_one ? (a[i] == 1) : (a[i] > 0)) {
            // C(n - 1, i) = 1 mod 2
            if (n - 1 == (i | (n - 1 - i))) {
                f = !f;
            }
        }
    }

    if (f) {
        cout << (has_one ? 1 : 2) << endl;
    } else {
        cout << 0 << endl;
    }
}