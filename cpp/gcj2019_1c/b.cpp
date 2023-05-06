#include <bits/stdc++.h>

using namespace std;

void execute(int64_t f) {
    constexpr int kNums[] = {23, 5, 1, 0};
    int num_ask = 0;

    string ans;
    vector<int> target_sets(119);
    iota(target_sets.begin(), target_sets.end(), 0);
    set<char> remains{'A', 'B', 'C', 'D', 'E'};
    for (int i_pos = 0; i_pos < 4; ++i_pos) {
        array<vector<int>, 5> s;
        for (auto i_set : target_sets) {
            ++num_ask;
            cout << i_set * 5 + i_pos + 1 << endl;

            char rep;
            cin >> rep;

            if (rep == 'N') {
                cerr << "Error" << endl;
                exit(1);
            }
            s[rep - 'A'].push_back(i_set);
        }

        int idx = -1;
        for (auto i = 0; i < 5; ++i) {
            if (remains.count('A' + i) == 0) {
                continue;
            }

            if (s[i].size() == kNums[i_pos]) {
                idx = i;
                break;
            }
        }

        if (idx == -1) {
            cerr << "Error: Not found" << endl;
            exit(1);
        }

        target_sets = move(s[idx]);
        ans.push_back('A' + idx);
        remains.erase(ans.back());
    }

    for (; num_ask < f; ++num_ask) {
        cout << 1 << endl;

        char rep;
        cin >> rep;
        if (rep == 'N') {
            cerr << "Error" << endl;
            exit(1);
        }
    }

    ans.push_back(*remains.begin());
    cout << ans << endl;

    char rep;
    cin >> rep;
    if (rep == 'N') {
        cerr << "Error: Incorrect" << endl;
        exit(1);
    }
}


int main() {
    int64_t t, f;
    cin >> t >> f;

    for (auto i = 0L; i < t; ++i) {
        execute(f);
    }

    return 0;
}