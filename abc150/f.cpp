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
int64_t kmp(const S& s, const S& w, const vector<int64_t>& t, int64_t i,
            int64_t j) {
    int64_t n = s.size(), m = w.size();

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

template <typename S>
int64_t kmp(const S& s, const S& w, const vector<int64_t>& t, int64_t i) {
    if (i == -1) {
        return kmp(s, w, t, 0, 0);
    } else {
        int64_t m = w.size();
        return kmp(s, w, t, i + m - t[m], i + m);
    }
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

    vector<uint64_t> c(n), d(n);
    for (auto i : irange(0L, n)) {
        c[i] = (a[i] ^ a[(i + 1) % n]);
        d[i] = (b[i] ^ b[(i + 1) % n]);
    }

    vector<int64_t> k;
    int64_t idx = -1;
    const auto& t = prekmp(d);
    while ((idx = kmp(c, d, t, idx)) >= 0) {
        k.push_back(idx);
    }

    for (auto kk : k) {
        cout << kk << " " << (a[kk] ^ b[0]) << "\n";
    }
}