#include <bits/stdc++.h>

#include <atcoder/convolution>
#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace atcoder;
using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n, m;
    cin >> n >> m;

    vector<int64_t> a(n), b(m);
    for (auto&& aa : a) {
        cin >> aa;
    }
    for (auto&& bb : b) {
        cin >> bb;
    }

    auto c = convolution(a, b);

    const auto* delim = "";
    for (auto cc : c) {
        cout << delim << cc;
        delim = " ";
    }

    cout << endl;
}