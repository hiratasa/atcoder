#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b;
    cin >> a >> b;

    for (auto i : {1, 2, 3}) {
        if (i != a && i != b) {
            cout << i << endl;
            return 0;
        }
    }
}