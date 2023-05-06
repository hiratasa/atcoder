#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

struct Query {
    int64_t a, b, c, d;
};

int64_t dfs(const vector<Query>& qs, vector<int64_t>& a, int64_t n, int64_t m) {
    if (a.size() == n) {
        int64_t p = 0;
        for (const auto& q : qs) {
            if (a[q.b] - a[q.a] == q.c) {
                p += q.d;
            }
        }

        return p;
    }

    int64_t ret = 0;
    for (auto i : irange(a.empty() ? 1L : a.back(), m + 1)) {
        a.push_back(i);
        ret = max(ret, dfs(qs, a, n, m));
        a.pop_back();
    }

    return ret;
}

int main() {
    int64_t n, m, q;
    cin >> n >> m >> q;

    vector<Query> qs(q);
    for (auto&& qq : qs) {
        cin >> qq.a >> qq.b >> qq.c >> qq.d;
        --qq.a;
        --qq.b;
    }

    vector<int64_t> a;
    cout << dfs(qs, a, n, m) << endl;
}