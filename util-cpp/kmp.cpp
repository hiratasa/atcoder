#include <bits/stdc++.h>

#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

// https://ja.wikipedia.org/wiki/%E3%82%AF%E3%83%8C%E3%83%BC%E3%82%B9%E2%80%93%E3%83%A2%E3%83%AA%E3%82%B9%E2%80%93%E3%83%97%E3%83%A9%E3%83%83%E3%83%88%E6%B3%95
// 部分文字列の探索アルゴリズム
// O(N + M)

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
        // w[i:j] == w[0:j-i]
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

template <typename S>
int64_t kmp(const S& s, const S& w) {
    const auto& t = prekmp(w);
    return kmp(s, w, t, 0, 0);
}

// template <typename S>
// vector<int64_t> kmp_all(const S& s, const S& w) {
//     const auto& t = prekmp(w);

//     vector<int64_t> ret;
//     int64_t i = -1;
//     while ((i = kmp(s, w, t, i)) >= 0) {
//         ret.push_back(i);
//     }

//     return ret;
// }

template <typename S>
vector<int64_t> kmp_all(const S& s, const S& w) {
    const auto& t = prekmp(w);

    vector<int64_t> ret;
    int64_t i = kmp(s, w, t, 0, 0);
    if (i < 0) {
        return {};
    }
    ret.push_back(i);
    while ((i = kmp(s, w, t, i + w.size() - t[w.size()], i + w.size())) >= 0) {
        ret.push_back(i);
    }

    return ret;
}

int main() {
    string s("ABC ABCDAB ABCDABCDABDEABCDABDFF");
    string w("ABCDABD");

    for (auto idx : kmp_all(s, w)) {
        cout << idx << endl;
    }
}