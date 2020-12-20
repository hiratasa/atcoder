#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    string s, t;
    cin >> s >> t;

    for (auto i : irange(0L, n)) {
        cout << s[i] << t[i];
    }
    cout << endl;
}