#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, k;
    bitset<18> s, t;
    cin >> n >> k;
    {
        uint64_t ss, tt;
        cin >> ss >> tt;

        s = ss;
        t = tt;
    }

    vector<bitset<18>> a;
    for (auto _ : irange(0L, n)) {
        uint64_t tmp;
        cin >> tmp;

        bitset<18> aa = tmp;

        if ((aa & s) != s) {
            continue;
        }

        if ((aa | t) != t) {
            continue;
        }

        a.push_back(aa);
    }

    n = a.size();

    vector<vector<int64_t>> combi(n + 1, vector<int64_t>(k + 1, 0));
    vector<vector<int64_t>> scombi(n + 1, vector<int64_t>(k + 1, 0));
    combi[0][0] = 1L;
    for (auto i : irange(0L, k + 1)) {
        scombi[0][i] = 1l;
    }
    for (auto i : irange(1L, n + 1)) {
        combi[i][0] = 1;
        for (auto j : irange(1L, min(i, k + 1))) {
            combi[i][j] = combi[i - 1][j - 1] + combi[i - 1][j];
        }
        if (i < k + 1) {
            combi[i][i] = 1;
        }

        scombi[i][0] = combi[i][0];
        for (auto j : irange(1L, k + 1)) {
            scombi[i][j] = scombi[i][j - 1] + combi[i][j];
        }
    }

    vector<int64_t> pos;
    for (auto i : irange(0L, 18L)) {
        if (!s[i] && t[i]) {
            pos.push_back(i);
        }
    }

    int64_t m = pos.size();

    int64_t ans = 0;
    for (auto b : irange(0uL, 1uL << m)) {
        bitset<18> bs(b);

        bitset<18> mask;
        int64_t c = 0;
        for (auto i : irange(0L, m)) {
            if (!bs[i]) {
                continue;
            }

            mask[pos[i]] = true;
            ++c;
        }

        unordered_map<uint64_t, int64_t> nums;
        for (const auto& aa : a) {
            ++nums[(aa & mask).to_ullong()];
        }

        int64_t tmp = 0;
        for (const auto& kv : nums) {
            tmp += scombi[kv.second][k] - scombi[kv.second][0];
        }

        if (c % 2 == 0) {
            ans += tmp;
        } else {
            ans -= tmp;
        }
    }

    cout << ans << endl;
}