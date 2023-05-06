#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int main() {
    int64_t n;
    cin >> n;

    bool has_odd = false;
    for (auto _ : irange(0L, n)) {
        int64_t a;
        cin >> a;

        if (a % 2 == 1) {
            has_odd = true;
        }
    }

    if (has_odd) {
        cout << "first" << endl;
    } else {
        cout << "second" << endl;
    }
}