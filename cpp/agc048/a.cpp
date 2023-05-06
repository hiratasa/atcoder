#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t t;
    cin >> t;

    for (auto _ : irange(0L, t)) {
        string s;
        cin >> s;

        constexpr int64_t C = 'z' - 'a' + 1;

        std::array<priority_queue<int64_t, vector<int64_t>, std::greater<>>, C>
                chars;
        for (auto i : irange(0uL, s.size())) {
            chars[s[i] - 'a'].emplace(i);
        }

        const auto* atcoder = "atcoder";
        constexpr auto K = 7L;

        constexpr auto INF = numeric_limits<int64_t>::max();
        int64_t ans = INF;
        vector<int64_t> idxs;
        int64_t r = 0;
        for (auto i : irange(0L, K + 1)) {
            auto c = atcoder[i];

            if (c == '\0') {
                c = -1;
            } else {
                c -= 'a';
            }

            int64_t idx = INF;
            for (auto cc = c + 1; cc < C; ++cc) {
                if (!chars[cc].empty()) {
                    idx = min(idx, chars[cc].top());
                }
            }

            if (idx != INF) {
                for (auto idx2 : idxs) {
                    if (idx < idx2) {
                        ++idx;
                    }
                }
                int64_t t = r + (idx - i);
                ans = min(ans, t);
            }

            if (c < 0 || chars[c].empty()) {
                break;
            }

            idx = chars[c].top();
            idxs.push_back(idx);
            for (auto idx2 : idxs) {
                if (idx < idx2) {
                    ++idx;
                }
            }
            chars[c].pop();
            r += idx - i;
        }

        if (ans == INF) {
            ans = -1;
        }
        cout << ans << "\n";
    }
}