#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    int s;
    cin >> s;

    int upper = s / 100;
    int lower = s % 100;

    bool is_yymm = true;
    bool is_mmyy = true;
    if (upper == 0 || upper > 12) {
        is_mmyy = false;
    }
    if (lower == 0 || lower > 12) {
        is_yymm = false;
    }

    if (is_yymm) {
        if (is_mmyy) {
            cout << "AMBIGUOUS" << endl;
        } else {
            cout << "YYMM" << endl;
        }
    } else {
        if (is_mmyy) {
            cout << "MMYY" << endl;
        } else {
            cout << "NA" << endl;
        }

    }
}