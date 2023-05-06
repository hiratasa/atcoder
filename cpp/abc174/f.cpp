#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

class BIT {
   public:
    explicit BIT(int64_t nn) : n(1 << (int64_t)ceil(log2(nn))), data(n + 1) {}

    // [0, i)
    int64_t query(int64_t i) {
        int64_t s = 0;

        while (i > 0) {
            s += data[i];
            i -= (i & -i);
        }

        return s;
    }

    void add(int64_t i, int64_t v) {
        int64_t idx = i + 1;

        while (idx <= n) {
            data[idx] += v;
            idx += (idx & -idx);
        }
    }

   private:
    int64_t n;
    vector<int64_t> data;
};

int main() {
    int64_t n, q;
    cin >> n >> q;

    vector<int64_t> c(n);
    for (auto&& cc : c) {
        cin >> cc;
    }

    vector<int64_t> prev(n + 1, -1);
    vector<std::tuple<int64_t, int64_t, int64_t>> intervals;

    for (auto i : irange(0L, q)) {
        int64_t l, r;
        cin >> l >> r;

        intervals.emplace_back(l - 1, i, r - 1);
    }

    for (auto i : irange(0L, n)) {
        int64_t cc = c[i];

        if (prev[cc] >= 0) {
            intervals.emplace_back(prev[cc], q, i);
        }

        prev[cc] = i;
    }

    sort(intervals.rbegin(), intervals.rend());

    BIT bit(n);
    vector<int64_t> ans(q, 0L);

    for (const auto [l, i_query, r] : intervals) {
        if (i_query == q) {
            bit.add(r, 1);
        } else {
            ans[i_query] = r - l + 1 - bit.query(r + 1);
        }
    }

    for (auto a : ans) {
        cout << a << "\n";
    }
}