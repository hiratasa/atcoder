#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    array<array<int64_t, 3>, 3> a;

    for (auto&& aa : a) {
        for (auto&& aaa : aa) {
            cin >> aaa;
        }
    }

    int64_t n;
    cin >> n;

    unordered_set<int64_t> s;
    for (auto i : irange(0L, n)) {
        int64_t b;
        cin >> b;
        s.insert(b);
    }

    for (auto&& aa : a) {
        for (auto&& aaa : aa) {
            if (s.count(aaa)) {
                aaa = -1;
            }
        }
    }

    bool bingo = false;
    for (auto i : irange(0L, 3L)) {
        if (a[i][0] == -1 && a[i][1] == -1 && a[i][2] == -1) {
            bingo = true;
        }
        if (a[0][i] == -1 && a[1][i] == -1 && a[2][i] == -1) {
            bingo = true;
        }
    }
    if (a[0][0] == -1 && a[1][1] == -1 && a[2][2] == -1) {
        bingo = true;
    }
    if (a[0][2] == -1 && a[1][1] == -1 && a[2][0] == -1) {
        bingo = true;
    }

    if (bingo) {
        cout << "Yes" << endl;
    } else {
        cout << "No" << endl;
    }
}