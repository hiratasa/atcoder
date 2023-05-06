#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    int64_t ans = 0;
    for (auto i : irange(0L, n - 2)) {
        if (s.substr(i, 3) == "ABC") {
            ++ans;
        }
    }

    cout << ans << endl;
}