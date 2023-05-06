#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int64_t n, x;
    cin >> n >> x;

    vector<int64_t> b(n), l(n), u(n);
    vector<pair<int64_t, int64_t>> p(n);
    int64_t score = 0;
    for (auto i : irange(0L, n)) {
        cin >> b[i] >> l[i] >> u[i];
        p[i] = make_pair(u[i] * x - (u[i] - l[i]) * b[i], i);
        score += l[i] * b[i];
    }

    sort(p.rbegin(), p.rend());

    cerr << score << endl;

    int64_t sum = 0;
    int64_t ans = 0;
    bool filled = false;
    vector<int64_t> sp;
    for (auto idx : irange(0L, n)) {
        const auto& pp = p[idx];
        // cerr << pp.second << " " << b[pp.second] << " " << l[pp.second] << " " << u[pp.second] << " " << l[pp.second] * b[pp.second] << " " << pp.first << endl;
        if (sum + pp.first > score) {
            if (!filled) {
                for (auto j = idx - 1; j >= 0; --j) {
                    auto r = score - (sum - p[j].first + pp.first);
                    auto i = p[j].second;
                    int64_t a = 0;
                    if (r < l[i] * b[i]) {
                        a = (r + l[i] - 1) / l[i];
                    } else {
                        a = b[i] + (r - l[i] * b[i] + u[i] - 1) / u[i];
                    }

                    // necessary??
                    if (a <= x) {
                        sp.push_back(a);
                    }
                }

            }

            filled = true;
            {
                auto r = score - sum;
                auto i = pp.second;
                int64_t a = 0;
                if (r < l[i] * b[i]) {
                    a = (r + l[i] - 1) / l[i];
                } else {
                    a = b[i] + (r - l[i] * b[i] + u[i] - 1) / u[i];
                }

                // necessary??
                if (a <= x) {
                    sp.push_back(a);
                }
            }
        } else if (filled) {
            break;
        } else {
            sum += pp.first;
            ans += x;

            if (sum == score) {
                break;
            }
        }
    }

    if (!sp.empty()) {
        ans += *min_element(sp.begin(), sp.end());
    }

    cout << ans << endl;
}