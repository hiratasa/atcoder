#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    int64_t n = s.size();

    if (s.front() != '1') {
        cout << -1 << endl;
        return 0;
    }
    if (s.back() != '0') {
        cout << -1 << endl;
        return 0;
    }

    for (auto i : irange(1L, n / 2 + 1)) {
        // 1-indexed
        if (s[i - 1] != s[n - i - 1]) {
            cout << -1 << endl;
            return 0;
        }
    }

    int64_t p = n, v = 1;
    for (auto i : irange(1L, n) | reversed) {
        // 1-indexed
        if (s[i - 1] == '0') {
            continue;
        }

        auto u = v;
        for (auto _ : irange(0L, p - i)) {
            cout << v << " " << ++u << "\n";
        }
        p = i;
        v = u;
    }
}