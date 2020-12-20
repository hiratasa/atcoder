#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

vector<int64_t> eval(const vector<int64_t>& a, int64_t k, int64_t b,
                     int64_t e) {
    if (e - b < k) {
        return {};
    }

    vector<int64_t> t;
    for (auto i : irange(b, e)) {
        t.push_back(a[i]);
    }

    sort(t.begin(), t.end());
    t.resize(t.size() - k + 1);

    return t;
}

int64_t eval(const vector<int64_t>& a, int64_t k, int64_t q,
             const vector<bool>& u) {
    int64_t n = a.size();

    vector<int64_t> t;
    int64_t b = 0;
    for (auto i : irange(0L, n + 1)) {
        if (!u[i]) {
            if (b != i) {
                auto tmp = eval(a, k, b, i);
                t.insert(t.end(), tmp.begin(), tmp.end());
            }

            b = i + 1;
        }
    }

    if (t.size() < q) {
        return numeric_limits<int64_t>::max();
    }

    sort(t.begin(), t.end());

    return t[q - 1] - t[0];
}

int main() {
    int64_t n, k, q;
    cin >> n >> k >> q;

    vector<int64_t> a(n);
    vector<pair<int64_t, int64_t>> b(n);
    for (auto i : irange(0L, n)) {
        cin >> a[i];
        b[i].first = a[i];
        b[i].second = i;
    }

    sort(b.begin(), b.end());

    int64_t ans = numeric_limits<int64_t>::max();
    vector<bool> u(n + 1, true);
    u[n] = false;
    for (auto i : irange(0L, n - k + 1)) {
        ans = min(ans, eval(a, k, q, u));
        u[b[i].second] = false;
    }

    cout << ans << endl;
}