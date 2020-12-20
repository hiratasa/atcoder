#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

class ST {
   public:
    using value_type = double;

    static constexpr auto K = 1L << 18;

    ST() : values_(2 * K, 0) {}

    void set(int64_t idx, value_type value) {
        idx += K - 1;
        values_[idx] = value;

        while (idx > 0) {
            idx = (idx - 1) / 2;
            values_[idx] = values_[2 * idx + 1] + values_[2 * idx + 2];
        }
    }

    value_type query(int64_t b, int64_t e) const {
        return query(b, e, 0, 0, K);
    }

    value_type query(int64_t b, int64_t e, int64_t idx, int64_t l,
                     int64_t r) const {
        if (e <= l || r <= b) {
            return 0;
        }

        if (b <= l && r <= e) {
            return values_[idx];
        }

        return query(b, e, 2 * idx + 1, l, (l + r) / 2) +
               query(b, e, 2 * idx + 2, (l + r) / 2, r);
    }

   private:
    std::vector<value_type> values_;
};

int main() {
    int64_t n;
    cin >> n;

    vector<pair<int64_t, int64_t>> pq(n);
    for (auto&& t : pq) {
        cin >> t.first >> t.second;
    }

    vector<double> log_fact(1L << 21, 0);
    for (auto i : irange(2uL, log_fact.size())) {
        log_fact[i] = log_fact[i - 1] + log(i);
    }

    auto combi = [&](int64_t a, int64_t b) {
        return log_fact[a] - log_fact[b] - log_fact[a - b];
    };

    ST st;
    for (auto i : irange(0L, n - 1)) {
        auto x = pq[i + 1].second - pq[i].second;
        auto y = pq[i + 1].first - pq[i].first;
        st.set(i, combi(x + y, x));
    }

    int64_t q;
    cin >> q;

    for (auto _ : irange(0L, q)) {
        int64_t t;
        cin >> t;

        if (t == 1) {
            int64_t k, a, b;
            cin >> k >> a >> b;
            --k;

            pq[k].first = a;
            pq[k].second = b;

            if (k > 0) {
                auto x = pq[k].second - pq[k - 1].second;
                auto y = pq[k].first - pq[k - 1].first;
                st.set(k - 1, combi(x + y, x));
            }
            if (k < n - 1) {
                auto x = pq[k + 1].second - pq[k].second;
                auto y = pq[k + 1].first - pq[k].first;
                st.set(k, combi(x + y, x));
            }
        } else {
            int64_t l1, r1, l2, r2;
            cin >> l1 >> r1 >> l2 >> r2;
            --l1;
            --r1;
            --l2;
            --r2;

            auto z1 = st.query(l1, r1);
            auto z2 = st.query(l2, r2);

            cout << (z1 > z2 ? "FIRST" : "SECOND") << "\n";
        }
    }
}