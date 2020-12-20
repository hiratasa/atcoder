#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    unordered_map<string, int64_t> dp;
    dp[""] = 1;

    for (auto i : irange(0L, n)) {
        decltype(dp) next;

        for (const auto& kv : dp) {
            const auto& key = kv.first;
            auto v = kv.second;

            // cerr << i << ":" << key << "=" << v << endl;

            auto last2 =
                    key.empty()
                            ? string()
                            : key.substr(
                                      max(static_cast<int64_t>(key.size()) - 2,
                                          0L),
                                      min(key.size(), 2uL));

            next[last2 + "A"] += v;
            if (last2 != "AC") {
                next[last2 + "G"] += v;
            }
            if (last2 != "AG" && last2 != "GA" &&
                !(key.size() == 3 && key.substr(0, 2) == "AG") &&
                !(key.size() == 3 && key[0] == 'A' && key[2] == 'G')) {
                next[last2 + "C"] += v;
            }
            next[last2 + "T"] += v;
        }

        for (auto&& kv : next) {
            kv.second %= 1000000007L;
        }

        dp = std::move(next);
    }

    int64_t ans = 0;
    for (const auto& kv : dp) {
        ans += kv.second;
        ans %= 1000000007L;
    }

    cout << ans << endl;
}