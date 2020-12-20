#include <iostream>

using namespace std;

int main() {
    int n, T;
    cin >> n >> T;

    int min_c = 1001;
    for (int i = 0; i < n; ++i) {
        int c, t;
        cin >> c >> t;
        if (t > T) {
            continue;
        }

        if (c < min_c) {
            min_c = c;
        }
    }

    if (min_c == 1001) {
        cout << "TLE" << endl;
    } else {
        cout << min_c << endl;
    }

    return 0;
}