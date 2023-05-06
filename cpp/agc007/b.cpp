#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> q(n);
    for (auto i : irange(0L, n)) {
        int64_t p;
        cin >> p;
        q[p - 1] = i + 1;
    }

    const auto* delim = "";
    for (auto i : irange(0L, n)) {
        cout << delim << n * i + q[i];
        delim = " ";
    }
    cout << endl;

    delim = "";
    for (auto i : irange(0L, n)) {
        cout << delim << n * (n - i);
        delim = " ";
    }
    cout << endl;
}