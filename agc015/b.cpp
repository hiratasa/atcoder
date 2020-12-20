#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    int64_t n = s.size();
    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        if (s[i] == 'U') {
            ans += 2 * i;
            ans += n - 1 - i;
        } else {
            ans += i;
            ans += 2 * (n - 1 - i);
        }
    }

    cout << ans << endl;
}