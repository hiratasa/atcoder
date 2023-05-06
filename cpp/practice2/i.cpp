#include <bits/stdc++.h>

#include <atcoder/string>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace atcoder;
using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    int64_t n = s.size();

    auto sa = suffix_array(s);
    auto lcp = lcp_array(s, sa);

    int64_t ans = n - sa[0];
    for (auto i : irange(0L, n - 1)) {
        auto l = lcp[i];
        ans += n - sa[i + 1] - l;
    }

    cout << ans << endl;
}