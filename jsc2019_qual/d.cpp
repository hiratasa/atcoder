#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t level(int64_t x, int64_t y) {
    assert(x != y);

    int64_t l = 0;
    while (((x >> l) ^ (y >> l)) % 2 == 0) {
        ++l;
    }

    return l + 1;
}

int main() {
    int64_t n;
    cin >> n;

    for (auto i : irange(0L, n - 1)) {
        const auto* delim = "";
        for (auto j : irange(i + 1, n)) {
            cout << delim << level(i, j);
            delim = " ";
        }
        cout << endl;
    }
}