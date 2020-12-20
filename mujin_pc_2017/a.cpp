#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> x(n);
    for (auto&& xx : x) {
        cin >> xx;
    }

    constexpr auto M = 1000000007L;

    int64_t ans = 1;
    int64_t k = 0, p = -1L;
    for (auto i : irange(0L, n)) {
        ans = ans * (k + 1) % M;
        if (p + 1 < x[i]) {
            p += 2;
            ++k;
        }
    }

    cout << ans << endl;
}