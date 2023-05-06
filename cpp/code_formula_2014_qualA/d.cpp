#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    constexpr int64_t M = '9' - '0' + 1 + 'z' - 'a' + 1;

    string s, k;
    cin >> s >> k;

    unordered_set<char> mem(k.begin(), k.end());
    unordered_set<char> not_mem_used;
    for (auto c : s) {
        if (mem.count(c) == 0) {
            not_mem_used.insert(c);
        }
    }

    int64_t d = not_mem_used.size();
    int64_t m = M - mem.size();

    double ans = s.size() - d;

    ans += 2 * (m - d) * d / (double)(d + 1);

    for (auto i : irange(0L, d)) {
        ans += 3 - 2 / (double)(i + 1);
    }

    cout << setprecision(20) << ans << endl;
}