#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

constexpr double EPS = 1.0e-12;

double dist(const pair<double, double>& p1, const pair<double, double>& p2) {
    auto dx = p1.first - p2.first;
    auto dy = p1.second - p2.second;
    return sqrt(dx * dx + dy * dy);
}

bool check(const vector<pair<double, double>>& p, const pair<double, double>& c,
           double r) {
    for (const auto& pp : p) {
        if (dist(pp, c) >= r + EPS) {
            return false;
        }
    }

    return true;
}

int main() {
    int64_t n;
    cin >> n;

    vector<pair<double, double>> p(n);
    for (auto&& pp : p) {
        cin >> pp.first >> pp.second;
    }

    double ans = numeric_limits<double>::max();
    for (auto i : irange(0L, n)) {
        for (auto j : irange(i + 1, n)) {
            pair<double, double> c;
            c.first = (p[i].first + p[j].first) / 2.0;
            c.second = (p[i].second + p[j].second) / 2.0;
            double r = dist(p[i], p[j]) / 2.0;

            if (check(p, c, r)) {
                ans = min(ans, r);
            }

            for (auto k : irange(j + 1, n)) {
                if (p[i].first == p[j].first && p[j].first == p[k].first) {
                    continue;
                }

                if (p[i].second == p[j].second && p[j].second == p[k].second) {
                    continue;
                }

                auto abcd = p[i].first * p[i].first +
                            p[i].second * p[i].second -
                            p[j].first * p[j].first - p[j].second * p[j].second;
                auto abef = p[i].first * p[i].first +
                            p[i].second * p[i].second -
                            p[k].first * p[k].first - p[k].second * p[k].second;

                pair<double, double> c;
                c.second = ((p[k].first - p[i].first) * abcd -
                            (p[j].first - p[i].first) * abef) /
                           (2 * (p[k].first - p[i].first) *
                                    (p[i].second - p[j].second) -
                            2 * (p[j].first - p[i].first) *
                                    (p[i].second - p[k].second));

                c.first = (p[i].first != p[j].first
                                   ? ((2 * (p[i].second - p[j].second) *
                                               c.second -
                                       abcd) /
                                      (2 * (p[j].first - p[i].first)))
                                   : ((2 * (p[i].second - p[k].second) *
                                               c.second -
                                       abef) /
                                      (2 * (p[k].first - p[i].first))));

                double r = dist(p[i], c);

                if (check(p, c, r)) {
                    ans = min(ans, r);
                }
            }
        }
    }

    cout << setprecision(10) << ans << endl;
}