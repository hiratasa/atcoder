#include <iostream>
#include <vector>
#include <string>
#include <numeric>
#include <cmath>
#include <cassert>
#include <algorithm>
#include <cstdint>
#include <unordered_map>
#include <unordered_set>

using namespace std;

int64_t solve(vector<int64_t>& a) {
    sort(a.begin(), a.end());

    int s = 0, t = a.size() - 1;
    int b = a[s], e = a[t];
    int64_t ans = e - b;
    while (s + 1 != t) {
        vector<int64_t> l{abs(a[s + 1] - b), abs(a[s + 1] - e), abs(a[t - 1] - b), abs(a[t - 1] - e)};

        auto it = max_element(l.begin(), l.end());
        ans += *it;
        auto idx = it - l.begin();
        if (idx <= 1) {
            ++s;
            if (idx == 0) {
                b = a[s];
            } else {
                e = a[s];
            }
        } else {
            --t;
            if (idx == 2) {
                b = a[t];
            } else {
                e = a[t];
            }
        }
    }

    return ans;
}

int main() {
    int n;
    cin >> n;

    vector<int64_t> a(n);
    for (auto&& aa : a) {
        cin >> aa;
    }

    cout << solve(a) << endl;

    return 0;
}