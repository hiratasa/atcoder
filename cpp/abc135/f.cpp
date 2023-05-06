#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

template <typename S>
vector<int64_t> prekmp(const S& w) {
    int64_t m = w.size();

    // t[k] = max_{i<k} w[k-i:k] == w[0:i]
    vector<int64_t> t(m + 1, -1);

    t[1] = 0;
    int64_t i = 1, j = 1;
    while (j < m) {
        assert(i <= j);
        // w[i:j] == w[0:j-i]
        if (w[j] == w[j - i]) {
            ++j;
            // w[i:j] == w[0:j-i]
            t[j] = j - i;
        } else if (i == j) {
            ++i;
            ++j;
            t[j] = 0;
        } else {
            // w[j-t[j-i]:j]
            //  == w'[j-i-t[j-i]:j-i] (where w' = w[i:j])
            //  == w[j-i-t[j-i]:j-i] (since w[i:j] = w[0:j-i])
            //  == w[0:t[j-i]]
            i = j - t[j - i];  // > i
        }
    }

    return t;
}

template <typename S>
int64_t kmp(const S& s, const S& w, const vector<int64_t>& t, int64_t i,
            int64_t j) {
    int64_t n = s.size(), m = w.size();

    while (j < n) {
        assert(i <= j);
        assert(j - i < m);
        // s[i:j] == w[0:j-i]
        if (s[j] == w[j - i]) {
            ++j;
            if (j - i == m) {
                return i;
            }
        } else if (i == j) {
            ++i;
            ++j;
        } else {
            // s[j-t[j-i]:j]
            //  == s'[j-i-t[j-i]:j-i] (where s' = s[i:j])
            //  == w[j-i-t[j-i]:j-i] (since s[i:j] = w[0:j-i])
            //  == w[0:t[j-i]]
            i = j - t[j - i];  // > i
        }
    }

    return -1;
}

constexpr auto INF = 1L << 60;

void dfs(vector<int64_t>& a, int64_t n, int64_t m, int64_t v) {
    auto u = (v + m) % n;

    if (a[u] == -1) {
        a[v] = 1;
    } else if (a[u] == -2 || a[u] == INF) {
        a[v] = INF;
    } else if (a[u] > 0) {
        a[v] = a[u] + 1;
    } else {
        a[v] = -2;
        dfs(a, n, m, u);
        if (a[u] == INF) {
            a[v] = INF;
        } else {
            a[v] = a[u] + 1;
        }
    }
}

int main() {
    string s, t;
    cin >> s >> t;

    while (s.size() < t.size()) {
        s += s;
    }

    int64_t n = s.size(), m = t.size();
    s += s;

    const auto& p = prekmp(t);

    int64_t idx = kmp(s, t, p, 0, 0);
    if (idx < 0) {
        cout << 0 << endl;
        return 0;
    }

    vector<int64_t> a(n, -1L);
    a[idx] = 0;
    while ((idx = kmp(s, t, p, idx + t.size() - p[t.size()], idx + t.size())) >=
           0) {
        if (idx >= n) {
            break;
        }
        a[idx] = 0;
    }

    int64_t ans = 0;
    for (auto i : irange(0L, n)) {
        if (a[i] >= 0) {
            dfs(a, n, m, i);
            ans = max(ans, a[i]);
        }
    }

    cout << (ans == INF ? -1 : ans) << endl;
}