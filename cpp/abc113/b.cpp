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

int solve(int t, int a, const vector<int>& h) {
    int ans = -1;
    int m = numeric_limits<int>::max();
    for (int i = 0; i < h.size(); ++i) {
        auto temp = t * 1000 - h[i] * 6;
        auto diff = abs(temp - 1000 * a);
        if (diff < m) {
            m = diff;
            ans = i + 1;
        }
    }

    return ans;
} 

int main() {
    int n;
    cin >> n;
    int t, a;
    cin >>t >> a;

    vector<int> h(n);
    for (auto&& hh : h) {
        cin >> hh;
    }

    cout << solve(t, a, h) << endl;

    return 0;
}