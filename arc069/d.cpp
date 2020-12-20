#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    string s;
    cin >> s;

    for (auto sheep0 : {false, true}) {
        for (auto sheep1 : {false, true}) {
            vector<bool> sheep(n);
            sheep[0] = sheep0;
            sheep[1] = sheep1;

            for (auto i : irange(1L, n - 1)) {
                sheep[i + 1] = (sheep[i - 1] ^ (sheep[i] != (s[i] == 'o')));
            }

            if ((sheep[0] == (s[0] == 'o')) != (sheep[1] == sheep[n - 1])) {
                continue;
            }

            if ((sheep[n - 1] == (s[n - 1] == 'o')) !=
                (sheep[n - 2] == sheep[0])) {
                continue;
            }

            for (auto i : irange(0L, n)) {
                cout << (sheep[i] ? 'S' : 'W');
            }
            cout << endl;
            return 0;
        }
    }

    cout << -1 << endl;
}