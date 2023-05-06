#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

void visit(vector<vector<int64_t>>& in_links, vector<vector<int64_t>>& out_links, vector<int64_t>& num_in_links, int64_t idx, vector<int64_t>& parents) {
    vector<int64_t> children;
    for (auto c : out_links[idx]) {
        --num_in_links[c];
        if (num_in_links[c] == 0) {
            children.push_back(c);
        }
    }

    for (auto c : children) {
        parents[c] = idx;
        visit(in_links, out_links, num_in_links, c, parents);
    }
}

main() {
    int64_t n, m;
    cin >> n >> m;

    auto k = n - 1 + m;

    vector<int64_t> num_in_links(n + 1);
    vector<vector<int64_t>> out_links(n + 1);
    vector<vector<int64_t>> in_links(n + 1);
    for (auto i : irange(0L, k)) {
        int64_t dep, arv;
        cin >> dep >> arv;
        ++num_in_links[arv];
        out_links[dep].push_back(arv);
        in_links[arv].push_back(dep);
    }

    int64_t root;
    for (auto i : irange(1L, n + 1)) {
        if (num_in_links[i] == 0) {
            root = i;
            break;
        }
    }

    vector<int64_t> parents(n + 1);
    visit(in_links, out_links, num_in_links, root, parents);

    for (auto i : irange(1L, n + 1)) {
        cout << parents[i] << endl;
    }
}