#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t q;
    cin >> q;

    for (auto _ : irange(0L, q)) {
        int64_t a, b;
        cin >> a >> b;

        if (a > b) {
            swap(a, b);
        }

        auto c = a * b;

        int64_t s = sqrt(a * b);

        int64_t ans = 2 * s;

        if (s * s == c) {
            ans -= 2;
            --s;
        } else if (s * (s + 1) >= c) {
            --ans;
        }

        if (a <= s) {
            --ans;
        }

        cout << ans << "\n";
    }
}