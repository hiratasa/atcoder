#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t sum_of_digits(int64_t n) {
    int64_t ret = 0;

    while (n > 0) {
        ret += n % 10;
        n /= 10;
    }

    return ret;
}

int main() {
    int64_t k;
    cin >> k;

    vector<int64_t> candidates;
    for (auto i : irange(1L, 1000L)) {
        int64_t d = i;
        for (auto j : irange(0L, 13L)) {
            candidates.push_back(d);
            d *= 10;
            d += 9;
        }
    }

    sort(candidates.begin(), candidates.end());
    candidates.erase(unique(candidates.begin(), candidates.end()),
                     candidates.end());

    sort(candidates.begin(), candidates.end(), [](int64_t n, int64_t m) {
        return make_pair(sum_of_digits(m) * n, n) <
               make_pair(sum_of_digits(n) * m, m);
    });

    int64_t m = 0;
    for (auto n : candidates) {
        if (m < n) {
            cout << n << "\n";
            m = n;
            --k;
            if (k == 0) {
                return 0;
            }
        }
    }
}