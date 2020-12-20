#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

// https://www.geeksforgeeks.org/z-algorithm-linear-time-pattern-searching-algorithm/
// Z-Arrayの構築
// O(N)
// for i < n, z[i] is max z[i] < n such that s[0:z[i]] = s[i:i+z[i]]
// パターンの検索にも使える

template <typename S>
vector<int64_t> z_algorithm(const S& s) {
    int64_t n = s.size();

    // z[i] = max_{j<n} s[0:j] = s[i:i+j]
    vector<int64_t> z(n, 0L);
    z[0] = n;

    int64_t l = 0, r = 0;
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

int main() {
    string s("ABC ABCDAB ABCDABCDABDEABCDABDFF");
    string w("ABCDABD");

    for (auto idx : find_all(s, w)) {
        cout << idx << endl;
    }
}