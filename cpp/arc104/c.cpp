#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    constexpr auto kUnknown = 1000L;

    vector<pair<int64_t, int64_t>> ab(n);
    vector<int64_t> l(2 * n, kUnknown);
    for (auto i : irange(0L, n)) {
        cin >> ab[i].first >> ab[i].second;

        --ab[i].first;
        --ab[i].second;

        if (ab[i].first >= 0) {
            l[ab[i].first] = i + 1;
        }
        if (ab[i].second >= 0) {
            l[ab[i].second] = -i - 1;
        }
    }

    vector<int64_t> c(2 * n, -1L);
    int64_t k = 0;
    for (auto i : irange(0L, 2 * n)) {
        if (l[i] == kUnknown) {
            continue;
        }

        if (l[i] > 0) {
            auto idx = l[i] - 1;

            if (ab[idx].second >= 0) {
                auto cc = ab[idx].second - ab[idx].first - 1;
                if (k > 0) {
                    // check consistency
                    if (cc != c[i - 1]) {
                        cout << "No" << endl;
                        return 0;
                    }
                }
                c[i] = cc;
                ++k;
            } else {
                if (k > 0) {
                    auto cc = c[i - 1];
                    if (l[i + cc + 1] != kUnknown) {
                        cout << "No" << endl;
                        return 0;
                    }
                    l[i + cc + 1] = -idx - 1;
                    ab[idx].second = i;
                    c[i] = cc;
                    ++k;
                }
            }
        } else {
            auto idx = -l[i] - 1;

            if (ab[idx].first >= 0) {
                assert(k > 0);
                --k;
                c[i] = ab[idx].second - ab[idx].first - 1;
            } else {
                if (k > 0) {
                    auto cc = c[i - 1];
                    if (l[i - cc - 1] != kUnknown) {
                        cout << "No" << endl;
                        return 0;
                    }
                    l[i - cc - 1] = idx + 1;
                    ab[idx].first = i;
                    c[i] = cc;
                }
            }
        }
    }

    cout << "Yes" << endl;
}