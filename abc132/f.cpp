#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, k;
    cin >> n >> k;

    constexpr auto M = 1000000007;

    int64_t s = sqrt(n);
    if ((s + 1) * (s + 1) <= n) {
        ++s;
    }

    // 1～j までの和
    vector<vector<int64_t>> counts(k + 1, vector<int64_t>(s + 1, 0));
    // 1～n/l までの和
    vector<vector<int64_t>> counts2(k + 1, vector<int64_t>(s + 1, 0));
    std::iota(counts[1].begin(), counts[1].end(), 0);
    for (int64_t l = 1; l <= s; ++l) {
        counts2[1][l] = n / l;
    }

    for (auto i : irange(2L, k + 1)) {
        for (auto j : irange(1L, s + 1)) {
            counts[i][j] = (counts[i][j - 1] + counts2[i - 1][j]) % M;
        }

        counts2[i][s] = (counts[i][s] + counts[i - 1][s] * (n / s - s) % M) % M;
        for (auto l = s - 1; l > 0; --l) {
            counts2[i][l] = (counts2[i][l + 1] +
                             (counts[i - 1][l] * (n / l - n / (l + 1))) % M) %
                            M;
        }
    }

    cout << counts2[k][1] << endl;
}