#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t pow10(int64_t n) {
    if (n == 0) {
        return 1;
    }

    auto p = pow10(n / 2);
    return p * p * ((n % 2) ? n : 1);
}

int64_t solve(string n, int64_t pos, int64_t k, bool f) {
    static vector<vector<vector<int64_t>>> cache(
            101, vector<vector<int64_t>>(4, vector<int64_t>(2, -1L)));

    if (cache[pos][k][f ? 1 : 0] >= 0) {
        return cache[pos][k][f ? 1 : 0];
    }

    if (pos == n.size()) {
        return (k > 0) ? 0 : 1;
    }

    int64_t ret = 0;
    if (f) {
        if (n[pos] == '0') {
            ret += solve(n, pos + 1, k, true);
        } else {
            if (k > 0) {
                ret += solve(n, pos + 1, k - 1, true);
                ret += ((n[pos] - '0') - 1) * solve(n, pos + 1, k - 1, false);
            }
            ret += solve(n, pos + 1, k, false);
        }
    } else {
        if (k == 0) {
            ret = 1;
        } else {
            ret += 9 * solve(n, pos + 1, k - 1, false);
            ret += solve(n, pos + 1, k, false);
        }
    }

    cache[pos][k][f ? 1 : 0] = ret;
    return ret;
}

int main() {
    string n;
    cin >> n;

    int64_t k;
    cin >> k;

    cout << solve(n, 0, k, true) << endl;
}