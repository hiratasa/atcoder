#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    constexpr auto M = 1000000007;
    using Key = std::tuple<int64_t, int64_t, int64_t>;
    unordered_map<Key, int64_t, boost::hash<Key>> m;
    // vector<vector<pair<Key, int64_t>>> v(n);
    m[Key{0, 0, 0}] = 1;
    // v[0].emplace_back(Key{0, 0, 0}, 0);
    // v[0].emplace_back(Key{0, 0, 0}, 1);
    // v[0].emplace_back(Key{0, 0, 0}, 2);
    for (auto p : a | indexed(0)) {
        auto i = p.index();
        auto aa = p.value();

        if (aa > i) {
            cout << 0 << endl;
            return 0;
        }

        decltype(m) m2;
        for (const auto& kv : m) {
            const auto& key = kv.first;
            auto num = kv.second;
            if (std::get<0>(key) == aa) {
                auto next_key =
                        std::make_tuple(std::get<0>(key) + 1, std::get<1>(key),
                                        std::get<2>(key));
                m2[next_key] += num;
                m2[next_key] %= M;
            }

            if (std::get<1>(key) == aa) {
                auto next_key =
                        std::make_tuple(std::get<0>(key), std::get<1>(key) + 1,
                                        std::get<2>(key));
                m2[next_key] += num;
                m2[next_key] %= M;
            }

            if (std::get<2>(key) == aa) {
                auto next_key =
                        std::make_tuple(std::get<0>(key), std::get<1>(key),
                                        std::get<2>(key) + 1);
                m2[next_key] += num;
                m2[next_key] %= M;
            }
        }

        m = std::move(m2);
    }

    int64_t ans = 0;
    for (const auto& kv : m) {
        ans += kv.second;
        ans %= M;
    }

    cout << ans << endl;
}