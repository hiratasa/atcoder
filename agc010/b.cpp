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
    int64_t sum = 0;
    for (auto&& aa : a) {
        cin >> aa;
        sum += aa;
    }

    if (sum % (n * (n + 1) / 2) > 0) {
        cout << "NO" << endl;
        return 0;
    }

    auto k = sum / (n * (n + 1) / 2);
    auto l = 0L;
    for (auto i : irange(0L, n)) {
        auto d = a[(i + 1) % n] - a[i];

        if ((k - d) % n > 0) {
            cout << "NO" << endl;
            return 0;
        }

        auto t = (k - d) / n;
        if (t < 0) {
            cout << "NO" << endl;
            return 0;
        }

        l += t;
    }

    if (l != k) {
        cout << "NO" << endl;
        return 0;
    }

    cout << "YES" << endl;
}