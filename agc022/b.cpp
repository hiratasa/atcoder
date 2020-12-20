#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    if (n == 3) {
        cout << "2 3 25" << endl;
    } else {
        const auto* delim = "";
        auto m = n / 8;
        for (auto i : irange(0L, m)) {
            cout << delim << 12 * i + 2;
            delim = " ";
            cout << delim << 12 * i + 3;
            cout << delim << 12 * i + 4;
            cout << delim << 12 * i + 6;
            cout << delim << 12 * i + 8;
            cout << delim << 12 * i + 9;
            cout << delim << 12 * i + 10;
            cout << delim << 12 * i + 12;
        }
        switch (n % 8) {
            case 0:
                cout << endl;
                break;
            case 1:
                cout << delim << 12 * m + 6 << endl;
                break;
            case 2:
                cout << delim << 12 * m + 2 << " " << 12 * m + 4 << endl;
                break;
            case 3:
                cout << delim << 12 * m + 2 << " " << 12 * m + 4 << " "
                     << 12 * m + 6 << endl;
                break;
            case 4:
                cout << delim << 12 * m + 2 << " " << 12 * m + 3 << " "
                     << 12 * m + 4 << " " << 12 * m + 9 << endl;
                break;
            case 5:
                cout << delim << 12 * m + 2 << " " << 12 * m + 3 << " "
                     << 12 * m + 4 << " " << 12 * m + 6 << " " << 12 * m + 9
                     << endl;
                break;
            case 6:
                cout << delim << 12 * m + 2 << " " << 12 * m + 3 << " "
                     << 12 * m + 4 << " " << 12 * m + 8 << " " << 12 * m + 9
                     << " " << 12 * m + 10 << endl;
                break;
            case 7:
                cout << delim << 12 * m + 2 << " " << 12 * m + 3 << " "
                     << 12 * m + 4 << " " << 12 * m + 8 << " " << 12 * m + 9
                     << " " << 12 * m + 10 << " " << 12 * m + 12 << endl;
                break;
        }
    }
}