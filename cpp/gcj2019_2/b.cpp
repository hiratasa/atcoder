#include <bits/stdc++.h>

using namespace std;

void solve() {
    constexpr auto M = 2;
    constexpr auto A = 1;

    int next = 1;
    vector<pair<int, pair<int, int>>> s;
    vector<int> count(21, 0);
    vector<bool> t(20, true);
    unordered_set<int> picked;
    s.emplace_back(1, make_pair(100, 20));
    picked.insert(100);

    for (int i = 1; i < 100 - M - A; ++i) {
        int c;
        cin >> c;
        if (c == -1) {
            cerr << "Turn number -1" << endl;
            exit(1);
        }

        while (!s.empty() && (!t[s.back().first] || s.back().first == s.back().second.second)) {
            ++s.back().first;

            if (s.back().first == 21) {
                s.pop_back();
            }
        }

        if (!s.empty()) {
            cout << s.back().first << " " << s.back().second.first << endl;
            ++count[s.back().first];
            ++s.back().first;

            if (s.back().first == 21) {
                s.pop_back();
            }
        } else {
            cout << next << " " << 0 << endl;

            int n;
            cin >> n;
            if (n == -1) {
                cerr << "Player number -1" << endl;
                exit(1);
            }
            for (int j = 0; j < n; ++j) {
                int m;
                cin >> m;
                if (picked.count(m) > 0) {
                    continue;
                }
                s.emplace_back(M + 1, make_pair(m, next));
                picked.insert(m);
            }

            if (n > i / 20) {
                t[next] = false;
            } else {
                t[next] = true;
            }

            ++next;
            if (next > M) {
                next = 1;
            }
        }
    }

    count.assign(M + 1, 0);
    for (int i = 0; i < M; ++i) {
        int c;
        cin >> c;
        if (c == -1) {
            cerr << "Turn Number -1" << endl;
            exit(1);
        }

        cout << i + 1 << " " << 0 << endl;

        int n;
        cin >> n;
        for (int j = 0; j < n; ++j) {
            int k;
            cin >> k;
        }

        count[i + 1] = n;
    }

    int last = max_element(count.begin() + 1, count.end()) - count.begin();
    for (int i = 0; i < A; ++i) {
        int c;
        cin >> c;
        if (c == -1) {
            cerr << "Last Turn Number -1" << endl;
            exit(1);
        }

        cout << last << " " << 1 << endl;
    }

    int c;
    cin >> c;
    if (c == -1) {
        cerr << "Last Turn Number -1" << endl;
        exit(1);
    }

    cout << 20 << " " << 100 << endl;
}

int main() {
    int t;
    cin >> t;

    for (int i = 0; i < t; ++i) {
        solve();
    }

    return 0;
}