#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

struct Mod {
    static constexpr auto kMod = 1000000007L;

    Mod() : n(0) {}
    // can be implicitly converted
    Mod(int64_t n) : n(n) {}

    Mod operator*(Mod m) const { return (n * (m.n % kMod)) % kMod; }

    Mod& operator*=(Mod m) {
        *this = *this * m;
        return *this;
    }

    Mod pow(int64_t p) {
        if (p == 0) {
            return 1;
        }
        if (p == 1) {
            return n;
        }

        int64_t r = this->pow(p / 2).n;
        if (p % 2 == 0) {
            return r * r % kMod;
        } else {
            return (r * r % kMod) * n % kMod;
        }
    }

    Mod operator/(Mod m) const {
        if (n == 0) {
            return 0;
        }

        return *this * m.pow(kMod - 2);
    }

    Mod& operator/=(Mod m) {
        *this = *this / m;
        return *this;
    }

    Mod operator+(Mod m) const { return (n + m.n) % kMod; }

    Mod& operator+=(Mod m) {
        *this = *this + m;
        return *this;
    }

    Mod operator-(Mod m) const { return (kMod + n - m.n) % kMod; }

    Mod& operator-=(Mod m) {
        *this = *this - m;
        return *this;
    }

    int64_t n;
};

int64_t dfs(const vector<vector<int64_t>>& adjs, vector<int64_t>& nums,
            int64_t v, int64_t p) {
    for (const auto& u : adjs[v]) {
        if (u == p) {
            continue;
        }

        nums[v] += dfs(adjs, nums, u, v);
    }

    return ++nums[v];
}

void solve(const vector<vector<int64_t>>& adjs, const vector<int64_t>& nums,
           vector<Mod>& ans, int64_t v, int64_t p) {
    int64_t n = adjs.size();
    for (const auto& u : adjs[v]) {
        if (u == p) {
            continue;
        }

        ans[u] = ans[v] * nums[u] / (n - nums[u]);
        solve(adjs, nums, ans, u, v);
    }
}

int main() {
    int64_t n;
    cin >> n;

    vector<vector<int64_t>> adjs(n);
    for (auto i : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        --a;
        --b;
        adjs[a].push_back(b);
        adjs[b].push_back(a);
    }

    vector<int64_t> nums(n);
    dfs(adjs, nums, 0, -1);

    vector<Mod> ans(n);
    ans[0] = 1;
    for (auto i : irange(0L, n)) {
        ans[0] *= i + 1;
        ans[0] /= nums[i];
    }

    solve(adjs, nums, ans, 0, -1);

    for (auto a : ans) {
        cout << a.n << "\n";
    }
}