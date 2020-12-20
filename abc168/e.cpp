#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    vector<pair<__int128_t, __int128_t>> ab;
    int64_t c = 0;
    for (auto i : irange(0L, n)) {
        int64_t a, b;
        cin >> a >> b;
        if (a == 0 && b == 0) {
            ++c;
        } else {
            if (b < 0 || (b == 0 && a < 0)) {
                a = -a;
                b = -b;
            }
            ab.emplace_back(a, b);
        }
    }

    sort(ab.begin(), ab.end(), [](const auto& lhs, const auto& rhs) {
        return lhs.first * rhs.second - lhs.second * rhs.first > 0;
    });

    constexpr auto M = 1000000007L;

    int64_t m = ab.size();

    vector<int64_t> p2(m + 1, 1L), ip2(m + 1, 1L);
    for (auto i : irange(1L, m + 1)) {
        p2[i] = p2[i - 1] * 2;
        p2[i] %= M;
        ip2[i] = ip2[i - 1] * 500000004L;
        ip2[i] %= M;
    }

    int64_t ans = 1L;
    for (int64_t i = 0, j = 0; i < m;) {
        int64_t i0 = i;
        while (i < m &&
               ab[i0].first * ab[i].second - ab[i0].second * ab[i].first == 0) {
            ++i;
        }
        while (j < m &&
               ab[i0].first * ab[j].first + ab[i0].second * ab[j].second < 0) {
            ++j;
        }
        int64_t j0 = j;
        while (j < m &&
               ab[i0].first * ab[j].first + ab[i0].second * ab[j].second == 0) {
            ++j;
        }

        ans += ans * ip2[j - j0] % M * (p2[i - i0] - 1) % M;
        ans %= M;
    }

    ans = (ans + c + M - 1) % M;

    cout << ans << endl;
}