#include <bits/stdc++.h>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t h, w, n;
    cin >> h >> w >> n;

    cout << min((n - 1) / w + 1, (n - 1) / h + 1) << endl;
}