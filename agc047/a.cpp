#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t count_i(int64_t k, int64_t p) {
    int64_t t = 0;
    while (k % p == 0) {
        ++t;
        k /= p;
    }

    return min(t, 18L);
}

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        double d;
        cin >> d;

        aa = llround(d * 1000000000L);
    }

    vector m(19, vector(19, 0L));
    for (auto aa : a) {
        auto b2 = count_i(aa, 2);
        auto b5 = count_i(aa, 5);

        ++m[b2][b5];
    }

    int64_t ans = 0;
    for (auto aa : a) {
        auto b2 = count_i(aa, 2);
        auto b5 = count_i(aa, 5);

        for (auto i2 : irange(18L - b2, 19L)) {
            for (auto i5 : irange(18L - b5, 19L)) {
                ans += m[i2][i5];
            }
        }

        if (b2 >= 9 && b5 >= 9) {
            --ans;
        }
    }

    assert(ans % 2 == 0);
    cout << ans / 2 << endl;
}
