#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

void dfs(const vector<vector<int64_t>>& links, vector<int64_t>& table, int64_t cur) {
    for (auto a : links[cur]) {
        if (table[a] >= 0) {
            continue;
        }

        table[a] = (table[cur] + 1) % 2;
        dfs(links, table, a);
    }
}

main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    vector<vector<int64_t>> links(n);
    for (auto _ : irange(0L, n - 1)) {
        int64_t a, b;
        cin >> a >> b;
        links[a - 1].push_back(b - 1);
        links[b - 1].push_back(a - 1);
    }

    vector<int64_t> table(n, -1);
    table[0] = 0;
    dfs(links, table, 0);

    auto m = find_if(s.begin(), s.end(), [](auto c) {
        return c == '1';
    }) - s.begin();

    for (auto i : irange(0L, n)) {
        if (s[i] == '1' && table[i] != table[m]) {
            cout << "-1" << endl;
            return 0;
        }
    }

    
}