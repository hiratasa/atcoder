#include <bits/stdc++.h>

using namespace std;

main() {
    int64_t t;
    cin >> t;
    
    for (auto i = 0l; i < t; ++i) {
        int64_t n, b, f;
        cin >> n >> b >> f;
        
        vector<int64_t> ss(n - b);
        for (auto j = 0; j < 5; ++j) {
            for (auto k = 0l; k < n; ++k) {
                cout << (((k % 32) >> j) & 1);
            }
            cout << endl;
            
            string s;
            cin >> s;
            for (auto k = 0l; k < n - b; ++k) {
                ss[k] += ((s[k] - '0') << j);
            }
        }
        
        vector<bool> v(n);
        for (auto k = 0l; k < n - b; ++k) {
            auto a = ss[k] % 32 - k % 32;
            if (a < 0) {
                a += 32;
            }
            v[k + a] = true;
        }
        
        const auto* delim = "";
        for (auto k = 0; k < n; ++k) {
            if (!v[k]) {
                cout << delim << k;
                delim = " ";
            }
        }
        cout << endl;

        int judge;
        cin >> judge;
        assert(judge == 1);
    }
}