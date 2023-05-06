#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(2 * n - 1);
    iota(a.begin(), a.end(), 1L);

    set<int64_t> s;
    do {
        auto b = a;
        for (auto i : irange(0L, n - 1)) {
            for (auto j : irange(0L, 2 * n - 1 - 2 * (i + 1))) {
                b[j] = b[j] + b[j + 1] + b[j + 2] -
                       min({b[j], b[j + 1], b[j + 2]}) -
                       max({b[j], b[j + 1], b[j + 2]});
            }
            b.resize(2 * n - 1 - 2 * (i + 1));
        }

        s.insert(b[0]);
    } while (next_permutation(a.begin(), a.end()));

    for (auto ss : s) {
        cout << ss << endl;
    }
}