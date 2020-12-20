#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s, t;
    cin >> s >> t;

    int64_t n = s.size();
    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        if (s[i] != t[i]) {
            ++ans;
        }
    }

    cout << ans << endl;
}