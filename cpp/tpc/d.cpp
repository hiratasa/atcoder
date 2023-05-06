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

int main() {
    int n;
    cin >> n;

    int k = sqrt(2 * n) + 1;

    if (k * (k - 1) / 2 != n) {
        cout << "No" << endl;
        return 0;
    }

    cout << "Yes" << endl;
    cout << k << endl;

    vector<vector<int>> rem(k);
    int current = 1;
    for (int i = 0; i < k; ++i) {
        cout << k - 1;
        for (int j = 0; j < i; ++j) {
            cout << " " << rem[j][i - j - 1];
        }
        while(rem[i].size() < k - 1 - i) {
            cout << " " << current;
            rem[i].push_back(current++);
        }

        cout << endl;
    }
    assert(current == n + 1);

    return 0;
}
