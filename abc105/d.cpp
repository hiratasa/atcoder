#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    unordered_map<int64_t, int64_t> nums;
    ++nums[0];
    int64_t ans = 0, s = 0;
    for (auto _ : irange(0L, n)) {
        int64_t a;
        cin >> a;
        s += a;
        ans += nums[s % m]++;
    }

    cout << ans << endl;
}