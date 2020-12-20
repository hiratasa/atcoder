#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t H, W, h, w;
    cin >> H >> W >> h >> w;

    if (H % h == 0 && W % w == 0) {
        cout << "No" << endl;
        return 0;
    }

    bool sw = false;
    if (H % h == 0) {
        std::swap(H, W);
        std::swap(h, w);
        sw = true;
    }

    vector<vector<int64_t>> a(H, vector<int64_t>(W));
    for (auto i : irange(0L, H / h)) {
        for (auto j : irange(0L, W)) {
            a[i * h][j] = H / h + 1;
            a[i * h + H % h][j] = -H / h - 2;
        }
    }
    for (auto j : irange(0L, W)) {
        a[H / h * h][j] = H / h + 1;
    }

    cout << "Yes" << endl;
    if (sw) {
        for (auto i : irange(0L, W)) {
            const auto* delim = "";
            for (auto j : irange(0L, H)) {
                cout << delim << a[j][i];
                delim = " ";
            }
            cout << "\n";
        }
    } else {
        for (auto i : irange(0L, H)) {
            const auto* delim = "";
            for (auto j : irange(0L, W)) {
                cout << delim << a[i][j];
                delim = " ";
            }
            cout << "\n";
        }
    }
}