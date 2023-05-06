#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

double distance(int64_t x1, int64_t y1, int64_t x2, int64_t y2) {
    auto dx = x1 - x2;
    auto dy = y1 - y2;

    return sqrt(dx * dx + dy * dy);
}

struct Circle {
    int64_t x, y, r;

    double distance_to(int64_t xx, int64_t yy) const {
        return max(0., distance(x, y, xx, yy) - r);
    }

    double distance_to(const Circle& other) const {
        return max(0., distance_to(other.x, other.y) - other.r);
    }
};

istream& operator>>(istream& is, Circle& c) { return is >> c.x >> c.y >> c.r; }

int main() {
    int64_t xs, ys, xt, yt;
    cin >> xs >> ys >> xt >> yt;

    int64_t n;
    cin >> n;

    vector<Circle> cs(n);
    for (auto&& cc : cs) {
        cin >> cc;
    }

    vector<vector<pair<int64_t, double>>> adjs(n + 2);
    adjs[0].emplace_back(n + 1, distance(xs, ys, xt, yt));
    for (auto i : irange(0L, n)) {
        // from start
        adjs[0].emplace_back(i + 1, cs[i].distance_to(xs, ys));

        // to target
        adjs[i + 1].emplace_back(n + 1, cs[i].distance_to(xt, yt));

        for (auto j : irange(i + 1, n)) {
            adjs[i + 1].emplace_back(j + 1, cs[i].distance_to(cs[j]));
            adjs[j + 1].emplace_back(i + 1, cs[i].distance_to(cs[j]));
        }
    }

    priority_queue<pair<double, int64_t>> q;
    vector<double> costs(n + 2, numeric_limits<double>::max());
    q.emplace(0L, 0L);
    costs[0] = 0;
    while (!q.empty()) {
        auto cost = -q.top().first;
        auto v = q.top().second;
        q.pop();

        if (cost > costs[v]) {
            continue;
        }

        if (v == n + 1) {
            cout << setprecision(20) << cost << endl;
            return 0;
        }

        for (const auto& edge : adjs[v]) {
            auto u = edge.first;
            auto new_cost = cost + edge.second;

            if (new_cost < costs[u]) {
                costs[u] = new_cost;
                q.emplace(-new_cost, u);
            }
        }
    }
}