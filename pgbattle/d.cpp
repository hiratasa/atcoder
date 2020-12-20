#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

constexpr int64_t M = 1000000007L;

main() {
    int64_t n;
    cin >> n;

    int64_t s = 0;
    int64_t score = 0;
    int64_t fact = 1;
    for (auto i : irange(1L, n + 1)) {
        fact = (fact * i) % M;
        score = (2 * s + (i - 1) * fact) % M;
        s = (score + i * s) % M;
    }

    cout << score << endl;
}