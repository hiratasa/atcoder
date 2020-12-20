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

    int64_t ans = 1;
    vector<int64_t> nums('z' - 'a' + 1);
    for (auto i : irange(0L, n)) {
        int64_t c = s[i] - 'a';

        ans += i - nums[c];

        ++nums[c];
    }

    cout << ans << endl;
}