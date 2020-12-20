#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

main() {
    int64_t n;
    cin >> n;

    constexpr auto U = 3001L;

    vector<pair<int64_t, int64_t>> xy(n);
    vector<int64_t> d(n);
    vector<vector<int64_t>> mm(U, vector<int64_t>(U));
    for (auto i : irange(0L, n)) {
        cin >> xy[i].first >> xy[i].second >> d[i];

        auto x = xy[i].first;
        auto y = xy[i].second;

        ++mm[x][y];
    }

    vector<vector<int64_t>> nums(3001, vector<int64_t>(6002));
    for (auto y : irange(0L, 6002L)) {
        for (auto x : irange(0L, 3001L)) {
            int64_t tmp = 0;
            if (x < mm.size() && y < mm[x].size()) {
                tmp += mm[x][y];
            }
            if (y >= 1) {
                if (x >= 1) {
                    tmp += nums[x - 1][y - 1];
                } else if (y >= 2) {
                    tmp += nums[x][y - 2];
                }
                if (x + 1 < nums.size()) {
                    tmp += nums[x + 1][y - 1];
                } else if (y - 2 >= 0) {
                    tmp += nums[x][y - 2];
                }
                if (y >= 2) {
                    tmp -= nums[x][y - 2];
                }
            }

            nums[x][y] = tmp;
        }
    }

    for (auto i : irange(0L, n)) {
        auto x = xy[i].first;
        auto y = xy[i].second;
        auto dd = d[i];

        if (dd == 0) {
            cout << mm[x][y] << "\n";
            continue;
        }

        if (dd == 1) {
            int64_t tmp = mm[x][y];
            if (y + 1 < mm[x].size()) {
                tmp += mm[x][y + 1];
            }
            if (x - 1 >= 0) {
                tmp += mm[x - 1][y];
            }
            if (x + 1 < mm.size()) {
                tmp += mm[x + 1][y];
            }
            if (y - 1 >= 0) {
                tmp += mm[x][y - 1];
            }

            cout << tmp << "\n";
            continue;
        }

        int64_t tmp = 0;
        if (y + dd - 1 >= 6000) {
            tmp += n;
        } else {
            tmp += nums[x][y + dd];
            if (y + dd - 1 >= 0) {
                tmp += nums[x][y + dd - 1];
            }
        }

        if (y >= 1) {
            if (x - dd >= 0) {
                tmp -= nums[x - dd][y - 1];
            } else if (y - 1 + (x - dd) >= 0) {
                tmp -= nums[0][y - 1 + (x - dd)];
            }
            if (x - dd - 1 >= 0) {
                tmp -= nums[x - dd - 1][y - 1];
            } else if (y - 1 + (x - dd - 1) >= 0) {
                tmp -= nums[0][y - 1 + (x - dd - 1)];
            }

            if (x + dd <= 3000) {
                tmp -= nums[x + dd][y - 1];
            } else if (y - 1 - (x + dd - 3000) >= 0) {
                tmp -= nums[3000][y - 1 - (x + dd - 3000)];
            }

            if (x + dd + 1 <= 3000 && y - 1 >= 0) {
                tmp -= nums[x + dd + 1][y - 1];
            } else if (y - 1 - (x + dd + 1 - 3000) >= 0) {
                tmp -= nums[3000][y - 1 - (x + dd + 1 - 3000)];
            }

            if (y - dd - 1 >= 0) {
                tmp += nums[x][y - dd - 1];
            }
            if (y - dd - 2 >= 0) {
                tmp += nums[x][y - dd - 2];
            }
        }

        cout << tmp << "\n";
    }
}