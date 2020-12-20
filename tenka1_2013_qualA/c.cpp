#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

void dfs(int64_t k, vector<vector<int64_t>>& rows, vector<int64_t>& row) {
    if (row.size() == k) {
        rows.push_back(row);
        return;
    }

    if (row.size() < 1 || row.back() != 1) {
        row.push_back(1);
        dfs(k, rows, row);
        row.pop_back();
    }

    if ((row.size() < 1 || row.back() != 2) &&
        (row.size() < 2 || row[row.size() - 2] != 2)) {
        row.push_back(2);
        dfs(k, rows, row);
        row.pop_back();
    }

    if ((row.size() < 1 || row.back() != 3) &&
        (row.size() < 2 || row[row.size() - 2] != 3) &&
        (row.size() < 3 || row[row.size() - 3] != 3)) {
        row.push_back(3);
        dfs(k, rows, row);
        row.pop_back();
    }
}

int64_t dfs2(int64_t k, const vector<vector<int64_t>>& rows,
             vector<int64_t>& col) {
    if (col.size() == k) {
        return 1;
    }

    int64_t ret = 0;

    for (auto i : irange(0uL, rows.size())) {
        bool ok = true;
        for (auto j : irange(0uL, rows[i].size())) {
            if (rows[i][j] == 1) {
                if (col.size() >= 1 && rows[col[col.size() - 1]][j] == 1) {
                    ok = false;
                    break;
                }
            } else if (rows[i][j] == 2) {
                if (col.size() >= 1 && rows[col[col.size() - 1]][j] == 2) {
                    ok = false;
                    break;
                }
                if (col.size() >= 2 && rows[col[col.size() - 2]][j] == 2) {
                    ok = false;
                    break;
                }
            } else if (rows[i][j] == 3) {
                if (col.size() >= 1 && rows[col[col.size() - 1]][j] == 3) {
                    ok = false;
                    break;
                }
                if (col.size() >= 2 && rows[col[col.size() - 2]][j] == 3) {
                    ok = false;
                    break;
                }
                if (col.size() >= 3 && rows[col[col.size() - 3]][j] == 3) {
                    ok = false;
                    break;
                }
            }
        }

        if (ok) {
            col.push_back(i);
            ret += dfs2(k, rows, col);
            col.pop_back();
        }
    }

    return ret;
}

int main() {
    int64_t m, n;
    cin >> m >> n;

    if (m >= 6) {
        m = (m - 6) % 4 + 6;
    }

    if (n >= 6) {
        n = (n - 6) % 4 + 6;
    }

    vector<vector<int64_t>> rows;
    vector<int64_t> row;
    dfs(m, rows, row);
    auto ans = dfs2(n, rows, row);

    cout << ans << endl;
}