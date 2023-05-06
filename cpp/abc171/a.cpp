#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/combine.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    char a;
    cin >> a;

    if (iswlower(a)) {
        cout << "a" << endl;
    } else {
        cout << "A" << endl;
    }
}