#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    constexpr auto K = 100001L;

    set<int64_t> primes;
    vector<int64_t> targets;
    vector<bool> is_prime(K, true);
    int64_t p = 2;
    for (p = 2L; p * p <= K; ++p) {
        if (is_prime[p]) {
            primes.insert(p);

            if (primes.count((p + 1) / 2)) {
                targets.push_back(p);
            }

            for (auto q = 2 * p; q < K; q += p) {
                is_prime[q] = false;
            }
        }
    }
    for (; p < K; ++p) {
        if (is_prime[p]) {
            primes.insert(p);

            if (primes.count((p + 1) / 2)) {
                targets.push_back(p);
            }
        }
    }

    int64_t q;
    cin >> q;
    for (auto _ : irange(0L, q)) {
        int64_t l, r;
        cin >> l >> r;

        cout << upper_bound(targets.begin(), targets.end(), r) -
                        lower_bound(targets.begin(), targets.end(), l)
             << endl;
    }
}