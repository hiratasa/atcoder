#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, l;
    cin >> n >> l;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    for (auto i : irange(0L, n - 1)) {
        if (a[i] + a[i + 1] >= l) {
            cout << "Possible" << endl;
            for (auto j : irange(0L, i)) {
                cout << j + 1 << "\n";
            }
            for (auto j : irange(i + 1, n - 1) | reversed) {
                cout << j + 1 << "\n";
            }
            cout << i + 1 << endl;
            return 0;
        }
    }

    cout << "Impossible" << endl;
}