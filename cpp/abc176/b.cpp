#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    string n;
    cin >> n;

    int64_t r = 0;
    for (auto c : n) {
        r += c - '0';
    }

    cout << (r % 9 == 0 ? "Yes" : "No") << endl;
}