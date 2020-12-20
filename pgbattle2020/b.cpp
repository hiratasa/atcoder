#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(int64_t n, int64_t s, int64_t cs, vector<int64_t>& ans) {
    if (ans.size() == n) {
        assert(s == cs);

        const auto* delim = "";
        for (auto a : ans) {
            cout << delim << a;
            delim = " ";
        }
        cout << "\n";
        return;
    }

    int64_t r = n - ans.size();
    int64_t rs = s - cs;
    int64_t p = ans.empty() ? 1 : ans.back();

    if (r * p > rs) {
        return;
    }

    if (r == 1) {
        ans.push_back(rs);
        dfs(n, s, cs + rs, ans);
        ans.pop_back();
        return;
    }

    for (auto i : irange(p, rs / r + 1)) {
        ans.push_back(i);
        dfs(n, s, cs + i, ans);
        ans.pop_back();
    }
}

int main() {
    int64_t n, s;
    cin >> n >> s;

    vector<int64_t> ans;
    dfs(n, s, 0, ans);
}