#include <bits/stdc++.h>

using namespace std;

int64_t gcd(int64_t a, int64_t b) {
    if (a < b) {
        swap(a, b);
    }

    int64_t r = a % b;

    if (r == 0) {
        return b;
    }

    return gcd(b, r);
}

pair<int64_t, int64_t> R(int64_t x, int64_t y) {
    auto g = gcd(x, y);

    return make_pair(x / g, y / g);
}

struct Hash {
    size_t operator()(const pair<int64_t, int64_t>& p) const {
        return p.first << 32 + p.second;
    }
};

int64_t solve(const vector<pair<int64_t, int64_t>>& p) {
    unordered_set<pair<int64_t, int64_t>, Hash> s;
    for (int64_t i = 0; i < p.size(); ++i) {
        for (int64_t j = i + 1; j < p.size(); ++j) {
            auto dx = p[i].first - p[j].first;
            auto dy = p[i].second - p[j].second;

            if (dx * dy >= 0) {
                continue;
            }

            s.insert(R(abs(dx), abs(dy)));
        }
    }

    return s.size() + 1;
}

int main() {
    int t;
    cin >> t;

    for (int i = 0; i < t; ++i) {
        int n;
        cin >> n;

        vector<pair<int64_t, int64_t>> p(n);
        for (auto&& pp : p) {
            cin >> pp.first >> pp.second;
        }

        auto ans = solve(p);

        cout << "Case #" << i + 1 << ": " << ans << endl;
    }

    return 0;
}