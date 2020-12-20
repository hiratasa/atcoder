#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s, t, u;
    int64_t a, b;
    cin >> s >> t >> a >> b >> u;

    if (s == u) {
        --a;
    } else {
        --b;
    }

    cout << a << " " << b << endl;
}