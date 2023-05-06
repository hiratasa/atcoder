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
    for (auto&& aa : a) {
        cin >> aa;

        if (aa == 0) {
            cout << 0 << endl;
            return 0L;
        }
    }

    int64_t s = 1;
    double sd = 1;
    for (auto aa : a) {
        s *= aa;
        sd *= aa;

        if (sd >= 2e18) {
            cout << -1 << endl;
            return 0;
        }
    }

    if (s > 1000000000000000000L) {
        cout << -1 << endl;
    } else {
        cout << s << endl;
    }
}