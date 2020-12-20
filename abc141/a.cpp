#include <bits/stdc++.h>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;

main() {
    string s;
    cin >> s;

    switch (s[0]) {
        case 'S':
            cout << "Cloudy" << endl;
            return 0;
        case 'C':
            cout << "Rainy" << endl;
            return 0;
        case 'R':
            cout << "Sunny" << endl;
            return 0;
    }
}