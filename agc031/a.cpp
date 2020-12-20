#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main(){
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    std::unordered_map<char, int64_t> counts;
    for (auto c : s) {
        ++counts[c];
    }

    constexpr auto M = 1000000007;

    int64_t ans = 1;
    for (auto kv : counts) {
        ans = ans * (kv.second + 1) % M;
    }

    // empty string
    --ans;
    if (ans < 0) {
        ans += M;
    }

    cout << ans << endl;
}