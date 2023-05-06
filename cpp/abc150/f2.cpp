#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

template <typename S>
vector<int64_t> prekmp(const S& w) {
    int64_t m = w.size();

    // t[k] = max_{t<k} w[k-t:k] == w[0:t]
    vector<int64_t> t(m + 1, -1);

    t[1] = 0;
    int64_t i = 1, j = 1;
    while (j < m) {
        assert(i <= j);
        // assert(w.substr(i, j - i) == w.substr(0, j - i));
        if (w[j] == w[j - i]) {
            ++j;
            // assert(w[i:j] == w[0:j-i])
            t[j] = j - i;
        } else if (i == j) {
            ++i;
            ++j;
            t[j] = 0;
        } else {
            // w[j-t[j-i]:j] == w[j-i-t[j-i]:j-i] ==w[0:t[j-i]]
            i = j - t[j - i];  // > i
        }
    }

    return t;
}

template <typename S>
int64_t kmp(const S& s, const S& w) {
    int64_t n = s.size(), m = w.size();

    const auto& t = prekmp(w);

    int64_t i = 1, j = 1;
    while (i < n) {
        assert(i <= j);
        assert(j - i < m);
        // assert(s.substr(i, j - i) == w.substr(0, j - i));
        if (s[j % n] == w[j - i]) {
            ++j;
            if (j - i == m) {
                return i;
            }
        } else if (i == j) {
            ++i;
            ++j;
        } else {
            // s[j-t[j-i]:j] == w[j-i-t[j-i]:j-i] ==w[0:t[j-i]]
            i = j - t[j - i];  // > i
        }
    }

    return -1;
}

int64_t period(const vector<uint64_t>& a) {
    auto p = kmp(a, a);
    if (p < 0) {
        return a.size();
    }

    return p;
}

int main() {
    int64_t n;
    cin >> n;

    vector<uint64_t> a(n), b(n);
    for (auto i : irange(0L, n)) {
        cin >> a[i];
    }
    for (auto i : irange(0L, n)) {
        cin >> b[i];
    }

    auto p = period(a);
    for (auto i : irange(0L, n)) {
        uint64_t x = (a[i] ^ b[0]);

        bool ok = true;
        for (auto j : irange(0L, n)) {
            if ((a[(i + j) % n] ^ x) != b[j]) {
                ok = false;
                break;
            }
        }

        if (!ok) {
            continue;
        }

        if (p % 2 == 0 && i + p / 2 < n) {
            uint64_t x2 = (a[i + p / 2] ^ b[0]);

            bool ok = true;
            for (auto j : irange(0L, n)) {
                if ((a[(i + j + p / 2) % n] ^ x2) != b[j]) {
                    ok = false;
                    break;
                }
            }

            if (ok) {
                p /= 2;
            }
        }

        for (auto j : irange(0L, n / p)) {
            uint64_t x = (a[i + j * p] ^ b[0]);
            cout << i + j * p << " " << x << "\n";
        }

        return 0;
    }
}