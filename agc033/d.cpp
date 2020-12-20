#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

int64_t execute(const vector<vector<bool>>& a, int64_t sr, int64_t sc, int64_t lr, int64_t lc) {
    

}

main() {
    int64_t h, w;
    cin >> h >> w;

    vector<vector<bool>> a(h);
    for (auto&& aa : a) {
        string s;
        cin >> s;
        aa.reserve(w);
        for (auto c : s) {
            aa.push_back(c == '#');
        }
    }

    execute(a);
}
