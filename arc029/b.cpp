#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    double a, b;
    cin >> a >> b;

    if (a < b) {
        swap(a, b);
    }

    int64_t n;
    cin >> n;

    vector<pair<double, double>> cd(n);
    for (auto&& t : cd) {
        cin >> t.first >> t.second;
        if (t.first < t.second) {
            swap(t.first, t.second);
        }
    }

    auto r = sqrt(a * a + b * b);
    auto phi = atan(a / b);

    for (const auto& t : cd) {
        auto c = t.first;
        auto d = t.second;
        if (a <= d) {
            cout << "YES\n";
            continue;
        }

        if (b > d) {
            cout << "NO\n";
            continue;
        }

        // if (b <= d && a <= c) {
        //     cout << "YES\n";
        //     continue;
        // }

        auto theta = M_PI - asin(d / r) - phi;
        if (theta > M_PI / 2.0 + 1e-10) {
            cout << "NO\n";
            continue;
        }

        auto e = min(a, a * sin(theta) + b * cos(theta));
        if (e > c) {
            cout << "NO\n";
            continue;
        }

        cout << "YES\n";
    }
}