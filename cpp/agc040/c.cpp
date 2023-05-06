#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    constexpr auto M = 998244353;

    int64_t num = 1, num2 = 0;
    for (int64_t i = 2; i <= n; i += 2) {
        auto next_num = 7 * num + 2 * 4 * num2;
        auto next_num2 = num + 7 * num2;
        num = next_num % M;
        num2 = next_num2 % M;
    }

    cout << num << endl;
}