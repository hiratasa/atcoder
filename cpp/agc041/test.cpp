#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int64_t quality(vector<int64_t>& q1, vector<int64_t>& q2) {
    int64_t q = q1.front();

    if (q <= 0) {
        return -1;
    }

    if (all_of(q1.begin(), q1.end(), [q](int64_t qq) { return q == qq; }) &&
        all_of(q2.begin(), q2.end(), [q](int64_t qq) { return q == qq; })) {
        return q;
    } else {
        return -1;
    }
}

void dump(vector<vector<int64_t>>& mat) {
    int64_t n = mat.size();
    cout.fill(' ');
    for (auto i : irange(0L, n)) {
        for (auto j : irange(0L, n)) {
            cout << setw(3) << mat[i][j];
        }
        cout << "\n";
    }
    cout << endl;
}

void dfs(vector<vector<int64_t>>& mat, vector<int64_t>& q1, vector<int64_t>& q2,
         int64_t next_idx, int64_t i, int64_t j) {
    int64_t n = mat.size();
    if (j == n) {
        if (i > 0) {
            if (q1[i] != q1[i - 1]) {
                return;
            }
        }

        ++i;
        j = 0;
    }
    if (i == n) {
        auto q = quality(q1, q2);
        if (q > 0) {
            cout << "Quaylity: " << q << endl;
            dump(mat);
        }
        return;
    }

    if (mat[i][j] >= 0) {
        dfs(mat, q1, q2, next_idx, i, j + 1);
        return;
    }

    dfs(mat, q1, q2, next_idx, i, j + 1);

    if (i + 1 < n) {
        mat[i][j] = mat[i + 1][j] = next_idx;
        ++q1[i];
        ++q1[i + 1];
        ++q2[j];
        dfs(mat, q1, q2, next_idx + 1, i, j + 1);
        --q2[j];
        --q1[i + 1];
        --q1[i];
        mat[i][j] = mat[i + 1][j] = -1;
    }

    if (j + 1 < n && mat[i][j + 1] < 0) {
        mat[i][j] = mat[i][j + 1] = next_idx;
        ++q1[i];
        ++q2[j];
        ++q2[j + 1];
        dfs(mat, q1, q2, next_idx + 1, i, j + 1);
        --q2[j + 1];
        --q2[j];
        --q1[i];
        mat[i][j] = mat[i][j + 1] = -1;
    }
}

int main() {
    int64_t n;
    cin >> n;

    vector<vector<int64_t>> mat(n, vector<int64_t>(n, -1));
    vector<int64_t> q1(n), q2(n);
    dfs(mat, q1, q2, 0, 0, 0);
}