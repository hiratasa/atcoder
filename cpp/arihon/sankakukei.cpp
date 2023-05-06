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

    vector a(n, 0L);
    for (auto&& aa : a) {
        cin >> aa;
    }

    sort(a.begin(), a.end());

    for (auto i : irange(2L, n) | reversed) {
        if (a[i] < a[i - 1] + a[i - 2]) {
            cout << a[i] + a[i - 1] + a[i - 2] << endl;
            return 0;
        }
    }

    cout << 0 << endl;
}