#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    cin >> n >> k;

    vector a(n, 0L);
    for (auto&& aa : a) {
        cin >> aa;
    }

    for (auto i : irange(k, n)) {
        if (a[i] > a[i - k]) {
            cout << "Yes\n";
        } else {
            cout << "No\n";
        }
    }
}