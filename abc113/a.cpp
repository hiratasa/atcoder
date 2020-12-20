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

int solve(int x, int y) {
    return x + y / 2;
} 

int main() {
    int x, y;
    cin >> x >> y;

    cout << solve(x, y) << endl;

    return 0;
}