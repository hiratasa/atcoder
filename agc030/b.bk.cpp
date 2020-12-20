#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t solve_greedy(const vector<int64_t>& x, int64_t l, int first, int last, int64_t pos) {
    if (first > last) {
        return 0;
    }

    int64_t dist_first = x[first] - pos;
    if (dist_first < 0) {
        dist_first += l;
    }

    int64_t dist_last = pos - x[last];
    if (dist_last < 0) {
        dist_last += l;
    }

#if 0
    if (dist_first < dist_last) {
        return dist_last + solve_greedy(x, l, first, last - 1, x[last]);
    } else if (dist_first > dist_last) {
        return dist_first + solve_greedy(x, l, first + 1, last, x[first]);
    } else {
        auto left_dist = solve_greedy(x, l, first + 1, last, x[first]);
        auto right_dist = solve_greedy(x, l, first, last - 1, x[last]);

        return dist_first + max(left_dist, right_dist);
    }
#else
    if (dist_first <= dist_last) {
        return dist_last + solve_greedy(x, l, first, last - 1, x[last]);
    } else {
        return dist_first + solve_greedy(x, l, first + 1, last, x[first]);
    }
#endif
}

struct Key {
    int64_t first;
    int64_t last;
    int64_t prev_dir;

    bool operator==(const Key& other) const {
        return first == other.first && last == other.last && prev_dir == other.prev_dir;
    }
};

struct KeyHasher {
    size_t operator()(const Key& key) const {
        auto h = (key.first << 32);
        h += key.prev_dir * key.last;
        return h;
    }
};

int64_t solve(const vector<int64_t>& x, int64_t l) {
    // static int64_t lower = solve_greedy(x, l, first, last, 0);

    vector<array<pair<int64_t, int>, 2>> prev(1, array<pair<int64_t, int>, 2>{make_pair(0L, 0), make_pair(0L, 0)});
    for (int len = x.size(); len >= 0; --len) {
        vector<array<pair<int64_t, int>, 2>> next(x.size());

        for (int first = 0; first + len - 1 < x.size(); ++first) {
            int last = first + len - 1;

            for (auto index : {0, 1}) {
                int64_t pos;
                if (index == 0) {
                    if (first == 0) {
                        if (len != x.size()) {
                            continue;
                        }
                        pos = 0;
                    } else {
                        pos = x[first - 1];
                    }
                } else {
                    if (last + 1 < x.size()) {
                        pos = x[last + 1];
                    } else {
                        if (len != x.size()) {
                            continue;
                        }
                        pos = 0;
                    }
                }

                auto previndex = (index == 0 ? first - 1 : last + 1 - (len - 2));
                if (len == x.size()) {
                    previndex = 0;
                }

                auto dist0 = (pos - prev[previndex][0].second);
                if (dist0 < 0) {
                    dist0 += l;
                }
                dist0 += prev[previndex][0].first;

                auto dist1 = prev[previndex][1].second - pos;
                if (dist1 < 0) {
                    dist1 += l;
                }
                dist1 += prev[previndex][1].first;

                next[first][index].first = max(dist0, dist1);
                next[first][index].second = pos;
            }
        }

        for (auto i : irange(0, (int)(x.size()) - len + 1)) {
            for (auto j : {0, 1}) {
                cout << i << ":" << j << " " << next[i][j].first << " " << next[i][j].second << endl;
            }
        }

        next.swap(prev);
    }

    int64_t ret = 0;
    for (const auto& p : prev) {
        for (const auto& pp : p) {
            ret = max(pp.first, ret);
        }
    }

    return ret;
}

main() {
    int64_t l, n;
    cin >> l >> n;

    vector<int64_t> x(n);
    for (auto&& xx : x) {
        cin >> xx;
    }

    cout << solve(x, l) << endl;
}