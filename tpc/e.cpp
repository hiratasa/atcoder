#include <iostream>
#include <vector>
#include <utility>
#include <string>
#include <numeric>
#include <cmath>
#include <cassert>
#include <algorithm>
#include <cstdint>
#include <unordered_map>
#include <unordered_set>

using namespace std;

struct P {
    int x;
    int y;

    void Swap(P& q) {
        auto tmp = *this;
        x = q.x;
        y = q.y;
        q.x = tmp.x;
        q.y = tmp.y;
    }
};

int64_t solve(vector<P>& ps) {
    vector<vector<int>> by_sum(600 + 1);
    vector<vector<int>> by_diff(600 + 1);
    constexpr auto kOffset = 300;

    auto get = [](const vector<int>& v, int idx) {
        if (idx < 0 || v.empty()) {
            return 0;
        }
        if (idx >= v.size()) {
            return v.back();
        }

        return v[idx];
    };

    sort(ps.begin(), ps.end(), [](const P& lhs, const P& rhs) {
        return lhs.x < rhs.x;
    });

    for (int i = 0; i < ps.size(); ++i) {
        const auto& p = ps[i];

        auto s = p.x + p.y;
        by_sum[s].resize(p.x + 1, by_sum[s].empty() ? 0 : by_sum[s].back());
        ++by_sum[s][p.x];

        auto d = p.x - p.y + kOffset;
        by_diff[d].resize(p.x + 1, by_diff[d].empty() ? 0 : by_diff[d].back());
        ++by_diff[d][p.x];
    }

    int64_t ans = 0;
    for (int s = 0; s < by_sum.size(); ++s) {
        for (int x = 0; x < by_sum[s].size(); ++x) {
            if (by_sum[s][x] == (x == 0 ? 0 : by_sum[s][x - 1])) {
                continue;
            }
            for (int x2 = x + 1; x2 < by_sum[s].size(); ++x2) {
                if (by_sum[s][x2] == by_sum[s][x2 - 1]) {
                    continue;
                }

                auto y = s - x;
                auto y2 = s - x2;

                auto dist = 2 * (x2 - x);

                {
                    auto s2 = (x + dist) + y;

                    auto x_first = x2;
                    auto x_last = x + dist;

                    if (s2 < by_sum.size()) {
                        const auto& t = by_sum[s2];
                        auto c = get(t, x_last) - get(t, x_first - 1);
                        if (c > 0) {
                            // cerr << "(" << x << "," << y << "), (" << x2 << ", " << y2 << ") : " << x_first << "->" << x_last << " with " << s2 << ": " << c << " (dist " << dist << ")" << endl;
                        }
                        ans += c;
                    }
                }

                {
                    auto s2 = (x2 - dist) + y2;

                    auto x_first = x2 - dist;
                    auto x_last = x;

                    if (s2 >= 0) {
                        const auto& t = by_sum[s2];
                        auto c = get(t, x_last) - get(t, x_first - 1);
                        if (c > 0) {
                            // cerr << "(" << x << "," << y << "), (" << x2 << ", " << y2 << ") : " << x_first << "->" << x_last << " with " << s2 << ": " << c << " (dist " << dist << ")" << endl;
                        }
                        ans += c;
                    }
                }
            }
        }
    }

    for (int d = 0; d < by_diff.size(); ++d) {
        auto diff = d - kOffset;
        for (int x = 0; x < by_diff[d].size(); ++x) {
            if (by_diff[d][x] == (x == 0 ? 0 : by_diff[d][x - 1])) {
                continue;
            }
            for (int x2 = x + 1; x2 < by_diff[d].size(); ++x2) {
                if (by_diff[d][x2] == by_diff[d][x2 - 1]) {
                    continue;
                }

                auto y = x - diff;
                auto y2 = x2 - diff;

                auto dist = 2 * (x2 - x);

                {
                    auto d2 = (x + dist) - y + kOffset;

                    // edge is already counted by sum, so +1/-1
                    auto x_first = x2 + 1;
                    auto x_last = x + dist - 1;

                    if (x_first <= x_last && d2 < by_diff.size()) {
                        const auto& t = by_diff[d2];
                        auto c = get(t, x_last) - get(t, x_first - 1);
                        if (c > 0) {
                            // cerr << "(" << x << "," << y << "), (" << x2 << ", " << y2 << ") : " << x_first << "->" << x_last << " with diff" << d2 << ": " << c << " (dist " << dist << ")" << endl;
                        }
                        ans += c;
                    }
                }

                {
                    auto d2 = (x2 - dist) - y2 + kOffset;

                    // edge is already counted by sum, so +1/-1
                    auto x_first = x2 - dist + 1;
                    auto x_last = x - 1;

                    if (x_first <= x_last && d2 >= 0) {
                        const auto& t = by_diff[d2];
                        auto c = get(t, x_last) - get(t, x_first - 1);
                        if (c > 0) {
                            // cerr << "(" << x << "," << y << "), (" << x2 << ", " << y2 << ") : " << x_first << "->" << x_last << " with diff" << d2 << ": " << c << " (dist " << dist << ")" << endl;
                        }
                        ans += c;
                    }
                }
            }
        }
    }

    return ans;
}

int main() {
    int h, w;
    cin >> h >> w;

    vector<P> ps;
    for (int y = 0; y < h; ++y) {
        for (int x = 0; x < w; ++x) {
            char c;
            cin >> c;
            if (c == '#') {
                ps.push_back({x, y});
            }
        }
    }

    cout << solve(ps) << endl;

    return 0;
}