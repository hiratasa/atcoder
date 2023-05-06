#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    std::array<int64_t, 4> a{};
    cin >> a[0] >> a[1] >> a[2] >> a[3];

    for (auto u : irange(1uL, 8uL)) {
        bitset<4> bs(u);

        std::array<int64_t, 2> s{};
        for (auto i : irange(0L, 4L)) {
            s[bs[i]] += a[i];
        }

        if (s[0] == s[1]) {
            cout << "Yes" << endl;
            return 0;
        }
    }

    cout << "No" << endl;
}