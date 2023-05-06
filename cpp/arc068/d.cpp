#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    unordered_map<int64_t, int64_t> m;
    for (auto _ : irange(0L, n)) {
        int64_t a;
        cin >> a;
        ++m[a];
    }

    int64_t t = 0;
    for (const auto& kv : m) {
        t += max(0L, kv.second - 1);
    }

    cout << n - (t + 1) / 2 * 2 << endl;
}