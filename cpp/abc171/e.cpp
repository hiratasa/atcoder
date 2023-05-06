#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector a(n, 0uL);
    uint64_t x = 0;
    for (auto&& aa : a) {
        cin >> aa;
        x ^= aa;
    }

    const auto* delim = "";
    for (auto aa : a) {
        cout << delim << (aa ^ x);
        delim = " ";
    }
    cout << endl;
}