#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string s;
    cin >> s;

    if (s.front() == s.back()) {
        if (s.size() % 2 == 0) {
            cout << "First" << endl;
        } else {
            cout << "Second" << endl;
        }
    } else {
        if (s.size() % 2 == 0) {
            cout << "Second" << endl;
        } else {
            cout << "First" << endl;
        }
    }
}