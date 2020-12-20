#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    int64_t x = 0;
    for (auto _ : irange(0L, n)) {
        int64_t a, k;
        cin >> a >> k;

        while (a >= k && a % k > 0) {
            auto r = a % k;
            auto p = a / k + 1;
            auto d = (r + p - 1) / p;
            a -= d * p;
        }

        int64_t g = a / k;

        x ^= g;
    }

    if (x > 0) {
        cout << "Takahashi" << endl;
    } else {
        cout << "Aoki" << endl;
    }
}