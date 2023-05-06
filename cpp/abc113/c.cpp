#include <iostream>
#include <vector>
#include <utility>
#include <string>
#include <numeric>
#include <cmath>
#include <cassert>
#include <iomanip>
#include <sstream>
#include <algorithm>
#include <cstdint>
#include <unordered_map>
#include <unordered_set>
#include <queue>

using namespace std;

vector<string> solve(const vector<int>& p, const vector<int>& y) {
    vector<int> indices(p.size());
    iota(indices.begin(), indices.end(), 0);
    sort(indices.begin(), indices.end(), [&y](int lhs, int rhs) {
        return y[lhs] < y[rhs];
    });


    vector<int> count(100001, 0);
    vector<string> ids(p.size());
    for (auto index : indices) {
        auto pp = p[index];
        auto yy = y[index];
        ++count[pp];

        ostringstream oss;
        oss << setw(6) << setfill('0') << pp;
        oss << setw(6) << setfill('0') << count[pp];
        ids[index] = oss.str();
    }

    return ids;
}

int main() {
    int n, m;
    cin >> n >> m;

    vector<int> p(m);
    vector<int> y(m);

    for (int i = 0; i < m; ++i) {
        cin >> p[i] >> y[i];
    }

    const auto& ans = solve(p, y);
    for (const auto& id : ans) {
        cout << id << endl;
    }

    return 0;
}