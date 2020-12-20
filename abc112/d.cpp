#include <iostream>
#include <vector>
#include <utility>
#include <cmath>

using namespace std;

int main() {
    int n, m;
    cin >> n >> m;

    int i = n;
    for (; i * i <= m; ++i) {
        if (m % i == 0) {
            cout << m / i << endl;;
            return 0;
        }
    }

    for (--i; i >= 1; --i) {
        if (m / i < n) {
            continue;
        }

        if (m % i == 0) {
            cout << i << endl;
            return 0;
        }
    }

    return 1;
}