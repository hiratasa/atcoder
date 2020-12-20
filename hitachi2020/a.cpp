#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    if (s.size() % 2 != 0) {
        cout << "No" << endl;
        return 0;
    }

    for (auto i : irange(0uL, s.size(), 2uL)) {
        if (s[i] != 'h' || s[i + 1] != 'i') {
            cout << "No" << endl;
            return 0;
        }
    }

    cout << "Yes" << endl;
}