#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    cin >> n;

    n %= 10;

    switch (n) {
        case 2:
        case 4:
        case 5:
        case 7:
        case 9:
            cout << "hon" << endl;
            return 0;
        case 0:
        case 1:
        case 6:
        case 8:
            cout << "pon" << endl;
            return 0;
        case 3:
            cout << "bon" << endl;
            return 0;
    }
}