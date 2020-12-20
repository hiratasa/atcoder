#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t update_h(char c, int64_t h) { return h * 29 + c; }

int64_t calc_h(const string_view& s) {
    int64_t h = 0;

    for (auto i : irange(0uL, s.size())) {
        h = update_h(s[i], h);
    }

    return h;
}

int main() {
    int64_t n;
    cin >> n;

    vector<string> s(n);
    for (auto&& ss : s) {
        cin >> ss;
        reverse(ss.begin(), ss.end());

        for (auto&& c : ss) {
            c -= 'a';
        }
    }

    sort(s.begin(), s.end(),
         [](const string& l, const string& r) { return l.size() < r.size(); });

    int64_t ans = 0;
    unordered_map<int64_t, vector<pair<string_view, array<int64_t, 26>>>> m;
    for (const auto& ss : s) {
        if (ss.size() >= 2) {
            array<int64_t, 26> v{};
            for (auto c : ss) {
                ++v[c];
            }

            int64_t h = 0;
            for (auto i : irange(0uL, ss.size() - 1)) {
                for (const auto& [sv, ar] : m[h]) {
                    if (sv != string_view(ss).substr(0, i)) {
                        continue;
                    }

                    for (auto c : irange(0L, 26L)) {
                        if (v[c] > 0) {
                            ans += ar[c];
                        }
                    }
                }

                --v[ss[i]];
                h = update_h(ss[i], h);
            }
        }

        bool updated = false;
        string_view r = string_view(ss).substr(0, ss.size() - 1);
        auto h = calc_h(r);
        for (auto&& kv : m[h]) {
            if (kv.first == r) {
                ++kv.second[ss.back()];
                updated = true;
                break;
            }
        }
        if (!updated) {
            m[h].emplace_back(r, array<int64_t, 26>{});
            ++m[h].back().second[ss.back()];
        }
    }

    cout << ans << endl;
}