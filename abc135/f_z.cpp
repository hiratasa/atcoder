#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

template <typename S>
vector<int64_t> z_algorithm(const S& s) {
    int64_t n = s.size();

    // z[i] = max_{j<n} s[0:j] = s[i:i+j]
    vector<int64_t> z(n, 0L);
    z[0] = n;

    int64_t l = 0, r = 1;
    for (auto i : irange(1L, n)) {
        if (i < r && z[i - l] < r - i) {
            z[i] = z[i - l];
        } else {
            // i < rなら、 z[i - l] >= r - i なので、
            // s[i:r] (=s[i-l:r-l]) = s[0:r-i] が保証されている
            // i >= r なら再計算
            l = i;
            r = max(i, r);
            while (r < n && s[r] == s[r - l]) {
                ++r;
            }
            z[i] = r - l;
        }
    }

    return z;
}

template <typename S>
vector<int64_t> find_all(const S& text, const S& pattern) {
    auto s = pattern;
    s.push_back('\0');
    s += text;

    auto z = z_algorithm(s);

    vector<int64_t> result;
    for (auto i : irange(0uL, text.size())) {
        if (z[pattern.size() + 1 + i] == pattern.size()) {
            result.push_back(i);
        }
    }

    return result;
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

    const auto& idxs = find_all(s, t);

    vector<int64_t> a(n, -1L);
    for (auto idx : idxs) {
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