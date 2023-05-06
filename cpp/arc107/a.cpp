#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b, c;
    cin >> a >> b >> c;

    constexpr auto M = 998244353;

    cout << (a * (a + 1) / 2 % M) * (b * (b + 1) / 2 % M) % M *
                    (c * (c + 1) / 2 % M) % M
         << endl;
}