#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector a(n, 0L);
    auto a_max = make_pair(numeric_limits<int64_t>::min(), -1L);
    for (auto i : irange(0L, n)) {
        cin >> a[i];
        a_max = max(a_max, make_pair(a[i], i));
    }

    if (a_max.first <= 0) {
        cout << a_max.first << endl;
        cout << n - 1 << endl;
        for (auto i : irange(0L, a_max.second)) {
            cout << 1 << "\n";
        }
        for (auto i : irange(a_max.second + 1, n) | reversed) {
            cout << i + 1 - a_max.second << "\n";
        }
        return 0;
    }

    array<int64_t, 2> s{}, l{-1L, -1L}, r{};
    for (auto i : irange(0L, n)) {
        if (a[i] > 0) {
            s[i % 2] += a[i];
            r[i % 2] = i;

            if (l[i % 2] == -1L) {
                l[i % 2] = i;
            }
        }
    }

    int64_t b = (s[0] < s[1] ? 1 : 0);

    cout << s[b] << endl;
    cout << l[b] + (n - r[b] - 1) + (r[b] - l[b]) / 2 << endl;
    for (auto i : irange(0L, r[b] + 1)) {
        if (i < l[b]) {
            cout << 1 << "\n";
        } else if (i % 2 != b) {
            // NOP
        } else if (i == l[b]) {
            // NOP
        } else if (a[i] <= 0) {
            cout << 3 << "\n";
        } else {
            cout << 2 << "\n";
        }
    }

    for (auto i : irange(r[b] + 1, n) | reversed) {
        cout << i - r[b] + 1 << "\n";
    }
}