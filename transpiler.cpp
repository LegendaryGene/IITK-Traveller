#include <bits/stdc++.h>
using namespace std;

int main(int argc, char* argv[]) {
    if(argc != 2){
        cout << "Wrong usage!" << endl;
        return 0;
    }

    freopen(argv[1], "r", stdin);
    freopen("output.iitktv", "w", stdout);

    vector<char> bf_code;
    char c;
    while(cin >> c){
        bf_code.push_back(c);
    }

    int cond = 0,mem_1 = 0, mem_2 = 1;
    int max_cond = 0;
    cout << "start, 0, oat_stage[1]" << endl;
    cond++;
    string prev_loc = "oat_stage";
    stack<int> cond_vals;

    for(auto code_char: bf_code) {
        switch (code_char)
        {
            case '>':
                if(mem_1 + 1 == mem_2) {
                    cout << prev_loc << ", " << cond << ", rm_2" << endl;
                    cout << "rm_2, " << cond << ", oat_stage[1]" << endl;
                    prev_loc = "oat_stage";
                    mem_2++;
                    cond++;
                }
                cout << prev_loc << ", " << cond << ", rm_1" << endl;
                cout << "rm_1, " << cond << ", oat_stage[1]" << endl;
                cond++;
                mem_1++;
                prev_loc = "oat_stage";
                max_cond = max(max_cond, cond);
                break;

            case '<':
                cout << prev_loc << ", " << cond << ", kd_1" << endl;
                cout << "kd_1, " << cond << ", oat_stage[1]" << endl;
                cond++;
                prev_loc = "oat_stage"; 
                mem_1--;
                max_cond = max(max_cond, cond);
                break;

            case '+':
                cout << prev_loc << ", " << cond << ", oat_stairs_1" << endl;
                cout << "oat_stairs_1, " << cond << ", oat_stage[1]" << endl;
                cond++;
                prev_loc = "oat_stage";
                max_cond = max(max_cond, cond);
                break;

            case '-':
                cout << prev_loc << ", " << cond << ", southern_labs_1" << endl;
                cout << "southern_labs_1, " << cond << ", oat_stage[1]" << endl;
                cond++;
                prev_loc = "oat_stage";
                max_cond = max(max_cond, cond);
                break;

            case '.':
                cout << prev_loc << ", " << cond << ", nankari_gate_out_1" << endl;
                cout << "nankari_gate_out_1, " << cond << ", oat_stage[1]" << endl;
                cond++;
                prev_loc = "oat_stage";
                max_cond = max(max_cond, cond);
                break;

            case ',':
                cout << prev_loc << ", " << cond << ", nankari_gate_in_1" << endl;
                cout << "nankari_gate_in_1, " << cond << ", oat_stage[1]" << endl;
                cond++;
                prev_loc = "oat_stage";
                max_cond = max(max_cond, cond);
                break;

            case '[':
                cout << prev_loc << ", " << cond << ", lecture_hall_eq" << endl;
                cond_vals.push(cond);
                cout << "lecture_hall_eq_f, " << cond << ", oat_stage[1]" << endl;
                cond++;
                prev_loc = "oat_stage";
                max_cond = max(max_cond, cond);
                break;

            case ']':
                if (cond_vals.empty()) {
                    cerr << "Error in brainfuck code" << endl;
                    return 0;
                }
                int prev_cond_val = cond_vals.top();
                cond_vals.pop();
                int cond_diff = prev_cond_val - cond;
                cout << prev_loc << ", " << cond << ", oat_stage[" << cond_diff << "]" << endl;
                cond = prev_cond_val;
                cond_diff = max_cond - cond + 1;
                cout << "lecture_hall_eq_t, " << cond << ", oat_stage[" << cond_diff << "]" << endl;
                cond = max_cond + 1;
                max_cond = max(max_cond, cond);
                prev_loc = "oat_stage";
                break;
        }
    }
    cout << prev_loc << ", " << cond << ", finish" << endl;
    return 0;
}