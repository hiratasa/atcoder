#include <bits/stdc++.h>

#include <boost/range/adaptors.hpp>
#include <boost/range/irange.hpp>

using namespace std;
using namespace boost;
using namespace boost::adaptors;

int main() {
    int64_t n;
    array<int64_t, 3> a;
    cin >> n >> a[0] >> a[1] >> a[2];

    vector<int64_t> ans;
    int64_t pi = -1, pj = -1;
    for (auto _ : irange(0L, n)) {
        string s;
        cin >> s;

        int64_t i = s[0] - 'A';
        int64_t j = s[1] - 'A';

        if (a[i] == 0 && a[j] == 0) {
            if (pi >= 0) {
                swap(a[pi], a[pj]);
                ans.back() = pj;
            }
        }

        if (a[i] == 0 && a[j] == 0) {
            cout << "No" << endl;
            return 0;
        }

        if (a[i] == 1 && a[j] == 1) {
            pi = i;
            pj = j;
            ++a[i];
            --a[j];
            ans.push_back(i);
            continue;
        }

        pi = -1;
        pj = -1;

        if (a[i] < a[j]) {
            ++a[i];
            --a[j];
            ans.push_back(i);
        } else {
            --a[i];
            ++a[j];
            ans.push_back(j);
            continue;
        }
    }

    cout << "Yes" << endl;
    for (auto i : ans) {
        cout << (char)(i + 'A') << "\n";
    }
}