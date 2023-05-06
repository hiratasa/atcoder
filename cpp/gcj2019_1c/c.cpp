#include <bits/stdc++.h>

using namespace std;

const auto kEmpty = numeric_limits<int>::max();
const auto kRadio = -1;

using Indexes = vector<pair<int, int>>;

Indexes PlaceHorizontal(const vector<vector<int>>& cells, int i_row, int i_col, int i_turn) {
    Indexes ret;

    if (cells[i_row][i_col] != kEmpty) {
        return Indexes{};
    }

    for (auto i = i_col; i < cells[i_row].size(); ++i) {
        if (cells[i_row][i] == kRadio) {
            return Indexes{};
        }

        if (cells[i_row][i] != kEmpty) {
            break;
        }

        ret.emplace_back(i_row, i);
    }

    for (auto i = i_col - 1; i >= 0; --i) {
        if (cells[i_row][i] == kRadio) {
            return Indexes{};
        }

        if (cells[i_row][i] != kEmpty) {
            break;
        }

        ret.emplace_back(i_row, i);
    }

    return move(ret);
}

Indexes PlaceVertical(const vector<vector<int>>& cells, int i_row, int i_col, int i_turn) {
    Indexes ret;

    if (cells[i_row][i_col] != kEmpty) {
        return Indexes{};
    }

    for (auto i = i_row; i < cells.size(); ++i) {
        if (cells[i][i_col] == kRadio) {
            return Indexes{};
        }

        if (cells[i][i_col] != kEmpty) {
            break;
        }

        ret.emplace_back(i, i_col);
    }

    for (auto i = i_row - 1; i >= 0; --i) {
        if (cells[i][i_col] == kRadio) {
            return Indexes{};
        }

        if (cells[i][i_col] != kEmpty) {
            break;
        }

        ret.emplace_back(i, i_col);
    }

    return move(ret);
}

void PrintCell(const vector<vector<int>>& cells, int i_turn) {
    cerr << i_turn << ":" << endl;

    for (const auto& v : cells) {
        for (auto i : v) {
            cerr << i << " ";
        }
        cerr << "\n";
    }

    cerr << endl;
}

int64_t execute(int64_t num_rows, int64_t num_cols, vector<vector<int>>& cells, int64_t i_turn) {
    // PrintCell(cells, i_turn);
    int64_t ret = 0;
    for (int64_t i_row = 0; i_row < num_rows; ++i_row) {
        for (int64_t i_col = 0; i_col < num_cols; ++i_col) {
            const auto hi = PlaceHorizontal(cells, i_row, i_col, i_turn);
            if (!hi.empty()) {
                for (const auto& p : hi) {
                    cells[p.first][p.second] = i_turn;
                }

                auto next_ret = execute(num_rows, num_cols, cells, i_turn + 1);

                if (next_ret == 0) {
                    ++ret;
                }

                for (const auto& p : hi) {
                    cells[p.first][p.second] = kEmpty;
                }
            }

            const auto vi = PlaceVertical(cells, i_row, i_col, i_turn);
            if (!vi.empty()) {
                for (const auto& p : vi) {
                    cells[p.first][p.second] = i_turn;
                }

                auto next_ret = execute(num_rows, num_cols, cells, i_turn + 1);

                if (next_ret == 0) {
                    ++ret;
                }

                for (const auto& p : vi) {
                    cells[p.first][p.second] = kEmpty;
                }
            }
        }
    }

    // cerr << i_turn << ":" << ret << endl;

    return ret;
}

int main() {
    int64_t t;
    cin >> t;

    for (auto i = 0L; i < t; ++i) {
        int64_t r, c;
        cin >> r >> c;

        vector<vector<int>> cells(r);
        for (auto&& row : cells) {
            string s;
            cin >> s;
            for (auto ch : s) {
                if (ch == '.') {
                    row.push_back(kEmpty);
                } else {
                    row.push_back(kRadio);
                }
            }
        }

        int64_t ans = execute(r, c, cells, 0);

        cout << "Case #" << i + 1 << ": " << ans << endl;
    }

    return 0;
}