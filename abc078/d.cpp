#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, z, w;
    cin >> n >> z >> w;

    vector<int64_t> a(n + 1, w);
    for (auto i : irange(0L, n)) {
        cin >> a[i + 1];
    }

    vector<vector<int64_t>> score(2, vector<int64_t>(n + 1));
    for (auto i : irange(1L, n + 1)) {
        score[0][i] = score[1][i] = abs(a[n] - a[n - i]);
        for (auto j : irange(1L, i)) {
            score[0][i] = max(score[0][i], score[1][j]);
            score[1][i] = min(score[1][i], score[0][j]);
        }
    }

    cout << score[0][n] << endl;
}