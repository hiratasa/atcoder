#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> nums(n, 1);
    vector<bool> has_red(n);
    has_red[0] = true;
    for (auto _ : irange(0L, m)) {
        int64_t x, y;
        cin >> x >> y;
        --x;
        --y;

        if (has_red[x]) {
            if (nums[x] == 1) {
                has_red[x] = false;
            }

            has_red[y] = true;
        }

        --nums[x];
        ++nums[y];
    }

    cout << count(has_red.begin(), has_red.end(), true) << endl;
}