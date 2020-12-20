#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

class BIT {
   public:
    explicit BIT(int64_t nn) : n(pow(2, ceil(log2(nn)))), b(n + 1) {}

    void init() { b.assign(n + 1, 0); }

    // [0, i)の和
    int64_t sum(int64_t i) const {
        int64_t s = 0;

        while (i > 0) {
            s += b[i];
            i -= (i & -i);
        }

        return s;
    }

    void add(int64_t i, int64_t a) {
        // 1-indexedに直す
        ++i;

        while (i <= n) {
            b[i] += a;
            i += (i & -i);
        }
    }

   private:
    int64_t n;
    vector<int64_t> b;
};

int main() {
    int64_t n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    vector<int64_t> r = a;
    sort(r.begin(), r.end());

    BIT bit(2 * n);
    auto ans = *partition_point(r.begin(), r.end(), [&](int64_t m) {
        vector<int64_t> b(n + 1);
        b[0] = n;
        for (auto i : irange(1L, n + 1)) {
            b[i] = b[i - 1] - 1;
            if (a[i - 1] <= m) {
                b[i] += 2;
            }
        }

        bit.init();
        bit.add(b[0], 1);
        int64_t k = 0;
        for (auto i : irange(1L, n + 1)) {
            k += bit.sum(b[i]);
            bit.add(b[i], 1);
        }

        return k <= n * (n + 1) / 2 / 2;
    });

    cout << ans << endl;
}