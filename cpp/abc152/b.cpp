#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t a, b;
    cin >> a >> b;

    if (a > b) {
        swap(a, b);
    }

    for (auto i : irange(0L, b)) {
        cout << a;
    }
    cout << endl;
}