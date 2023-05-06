#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t k, a, b;
    cin >> k >> a >> b;

    for (auto i : irange(a, b + 1)) {
        if (i % k == 0) {
            cout << "OK" << endl;
            return 0;
        }
    }

    cout << "NG" << endl;
}