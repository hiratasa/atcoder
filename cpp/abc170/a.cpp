#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    std::array<int64_t, 5> x;
    for (auto&& xx : x) {
        cin >> xx;
    }

    for (auto i : irange(0L, 5L)) {
        if (x[i] == 0) {
            cout << i + 1 << endl;
            return 0;
        }
    }
}