#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t k;
    cin >> k;

    string s;
    cin >> s;

    if (s.size() > k) {
        s.resize(k);
        s += "...";
    }

    cout << s << endl;
}