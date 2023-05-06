#include <bits/stdc++.h>

#include <boost/optional.hpp>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

struct Prefix {
    int64_t i_s = 0;
    int64_t len = 0;
};

struct EqualPrefix {
    bool operator()(const Prefix& lhs, const Prefix& rhs) const {
        return string_view(s[lhs.i_s]).substr(0, lhs.len) ==
               string_view(s[rhs.i_s]).substr(0, rhs.len);
    }

    const vector<string>& s;
};

struct HashPrefix {
    size_t operator()(const Prefix& p) const {
        static vector<vector<boost::optional<size_t>>> memo(s.size());

        if (memo[p.i_s].empty()) {
            memo[p.i_s].resize(s[p.i_s].size() + 1, boost::none);
            memo[p.i_s][0] = 0;
        }

        if (memo[p.i_s][p.len] == boost::none) {
            memo[p.i_s][p.len] = (*this)(Prefix{p.i_s, p.len - 1}) * 29 +
                                 s[p.i_s][p.len - 1];
        }

        return *memo[p.i_s][p.len];
    }

    const vector<string>& s;
};

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
    unordered_map<Prefix, array<int64_t, 26>, HashPrefix, EqualPrefix> m(
            0, HashPrefix{s}, EqualPrefix{s});
    for (auto i_s : irange(0L, n)) {
        const auto& ss = s[i_s];
        if (ss.size() >= 2) {
            array<int64_t, 26> v{};
            for (auto c : ss) {
                ++v[c];
            }

            for (auto i : irange(0uL, ss.size() - 1)) {
                Prefix p{i_s, i};

                if (m.count(p) > 0) {
                    const auto& ar = m[p];
                    for (auto c : irange(0L, 26L)) {
                        if (v[c] > 0) {
                            ans += ar[c];
                        }
                    }
                }

                --v[ss[i]];
            }
        }

        ++m[Prefix{i_s, ss.size() - 1}][ss.back()];
    }

    cout << ans << endl;
}