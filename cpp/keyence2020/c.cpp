#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k, s;
    cin >> n >> k >> s;

    if (s == 1000000000L) {
        const auto* delim = "";
        for (auto i : irange(0L, k)) {
            cout << delim << s;
            delim = " ";
        }
        for (auto i : irange(k, n)) {
            cout << delim << s - 1;
            delim = " ";
        }
    } else {
        const auto* delim = "";
        for (auto i : irange(0L, k)) {
            cout << delim << s;
            delim = " ";
        }
        for (auto i : irange(k, n)) {
            cout << delim << s + 1;
            delim = " ";
        }
    }
    cout << endl;
}