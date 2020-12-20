#include <iostream>
#include <vector>
#include <utility>
#include <cmath>

using namespace std;

struct D {
    int x;
    int y;
    int h;
};

int main() {
    int n;
    cin >> n;
    
    int first_non_zero = -1;
    vector<D> ds(n);
    for (int i = 0; i < n; ++i) {
        auto& d = ds[i];
        cin >> d.x >> d.y >> d.h;

        if (d.h > 0 && first_non_zero < 0) {
            first_non_zero = i;
        }
    }

    if (first_non_zero < 0) {
        return 1;
    }

    for (int c_x = 0; c_x <= 100; ++c_x) {
        for (int c_y = 0; c_y <= 100; ++c_y) {
            int xx = abs(ds[first_non_zero].x - c_x);
            int yy = abs(ds[first_non_zero].y - c_y);
            int h = ds[first_non_zero].h;
            int H = h + xx + yy;

            bool ok = true;
            for (const auto& d : ds) {
                int h = max(H - abs(d.x - c_x) - abs(d.y - c_y), 0);
                if (h != d.h) {
                    ok = false;
                    break;
                }
            }

            if (ok) {
                cout << c_x << " " << c_y << " " << H;
                return 0;
            }
        }
    }

    return 1;
}