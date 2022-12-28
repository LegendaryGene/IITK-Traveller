#include <bits/stdc++.h>

#include <boost/algorithm/string.hpp>

#include "operations.h"

using namespace boost::algorithm;
using namespace std;

vector<vector<string>> lines;
vector<map<int, int>> graph(num_ops);
int curr_lm;

map<string, int> ops = {
    {"start", start},
    {"finish", finish},
    {"iit_gate_in_1", iit_gate_in_1},
    {"iit_gate_in_2", iit_gate_in_2},
    {"hall_2", hall_2},
    {"hall_3", hall_3},
    {"hall_5", hall_5},
    {"hall_12", hall_12},
    {"mt_1_3", mt_1_3},
    {"mt_3_1", mt_3_1},
    {"mt_2_3", mt_2_3},
    {"mt_3_2", mt_3_2},
    {"iit_gate_out_1", iit_gate_out_1},
    {"iit_gate_out_2", iit_gate_out_2},
    {"doaa_1", doaa_1},
    {"doaa_2", doaa_2},
    {"lecture_hall_gt", lecture_hall_gt},
    {"lecture_hall_gt_t", lecture_hall_gt_t},
    {"lecture_hall_gt_f", lecture_hall_gt_f},
    {"lecture_hall_lt", lecture_hall_lt},
    {"lecture_hall_lt_t", lecture_hall_lt_t},
    {"lecture_hall_lt_f", lecture_hall_lt_f},
    {"lecture_hall_eq", lecture_hall_eq},
    {"lecture_hall_eq_t", lecture_hall_eq_t},
    {"lecture_hall_eq_f", lecture_hall_eq_f},
    {"oat_stairs_1", oat_stairs_1},
    {"oat_stairs_2", oat_stairs_2},
    {"oat_stairs_c", oat_stairs_c},
    {"southern_labs_1", southern_labs_1},
    {"southern_labs_2", southern_labs_2},
    {"southern_labs_c", southern_labs_c},
    {"hall_13_1", hall_13_1},
    {"hall_13_2", hall_13_2},
    {"hall_13_3", hall_13_3},
    {"hall_13_c", hall_13_c},
    {"rm_1", rm_1},
    {"rm_2", rm_2},
    {"rm_3", rm_3},
    {"kd_1", kd_1},
    {"kd_2", kd_2},
    {"kd_3", kd_3},
    {"eshop_1", eshop_1},
    {"eshop_2", eshop_2},
};

vector<int> mem(2048, 0);
int mem_1 = 0, mem_2 = 1, mem_3 = 2;
int cond = 0;

int input(char* file_name) {
    ifstream program_file(file_name);

    string program_line;
    int line_number = 0;

    while (getline(program_file, program_line)) {
        trim(program_line);
        int eol = 0;

        if (program_line.size() > 0) lines.push_back(vector<string>(0));

        string curr_word = "";

        for (auto c : program_line) {
            if (eol == 1) {
                cout << "Syntax error in line number " << line_number + 1
                     << endl;
                return 1;
            }

            if (c == ',') {
                lines[line_number].push_back(curr_word);
                curr_word = "";
                continue;
            }

            if (c == ';') {
                lines[line_number].push_back(curr_word);
                curr_word = "";
                eol = 1;
                continue;
            }

            curr_word += c;
        }

        if (eol != 1) {
            printf("Line %d: missing ;\n", line_number);
            return 1;
        }

        line_number++;
    }

    for (int line = 0; line < lines.size(); line++) {
        if (lines[line].size() != 3) {
            cout << "Syntax error in line number " << line + 1 << endl;
            return 1;
        }

        for (int i = 0; i < 3; i++) trim(lines[line][i]);
    }

    return 0;
}

int graph_builder(void) {
    for (int line = 0; line < lines.size(); line++) {
        string lm_1 = lines[line][0];
        string lm_2 = lines[line][2];
        int cond_val;

        try {
            cond_val = stoi(lines[line][1]);
        } catch (...) {
            printf("Line %d: Given weight is not a valid integer value \n",
                   line + 1);
            return 1;
        }

        if (ops.find(lm_1) == ops.end() || ops.find(lm_2) == ops.end()) {
            printf("Line %d: Given landmark is not a valid landmark\n",
                   line + 1);
            return 1;
        }

        if (graph[ops[lm_1]].find(cond_val) != graph[ops[lm_1]].end()) {
            printf(
                "Line %d: Path with the given condition value already exists "
                "from the landmark\n",
                line + 1);
            return 1;
        }

        graph[ops[lm_1]][cond_val] = ops[lm_2];
    }

    return 0;
}

void operations(int operation) {
    switch (operation) {
        case iit_gate_in_1:
            cin >> mem[mem_1];
            break;
        case iit_gate_in_2:
            cin >> mem[mem_2];
            break;
        case hall_2:
            mem[mem_3] = mem[mem_1] + mem[mem_2];
            break;
        case hall_3:
            mem[mem_3] = mem[mem_1] * mem[mem_2];
            break;
        case hall_5:
            mem[mem_3] = mem[mem_1] - mem[mem_2];
            break;
        case hall_12:
            mem[mem_3] = mem[mem_1] / mem[mem_2];
            break;

        case mt_1_3:
            mem[mem_1] = mem[mem_3];
            break;
        case mt_3_1:
            mem[mem_3] = mem[mem_1];
            break;
        case mt_2_3:
            mem[mem_2] = mem[mem_3];
            break;
        case mt_3_2:
            mem[mem_3] = mem[mem_2];
            break;

        case iit_gate_out_1:
            cout << mem[mem_1] << endl;
            break;
        case iit_gate_out_2:
            cout << mem[mem_2] << endl;
            break;
        case doaa_1:
            cout<< (char)mem[mem_1] << endl;
            break;
        case doaa_2:
            cout<< (char)mem[mem_2] << endl;
            break;
        case lecture_hall_gt:
            if (mem[mem_1] > mem[mem_2])
                curr_lm = lecture_hall_gt_t;
            else
                curr_lm = lecture_hall_gt_f;
            break;
        case lecture_hall_lt:
            if (mem[mem_1] < mem[mem_2])
                curr_lm = lecture_hall_lt_t;
            else
                curr_lm = lecture_hall_lt_f;
            break;
        case lecture_hall_eq:
            if(mem[mem_1] == mem[mem_2])
                curr_lm = lecture_hall_eq_t;
            else
                curr_lm = lecture_hall_eq_f;
            break;

        case oat_stairs_1:
            mem[mem_1]++;
            break;
        case oat_stairs_2:
            mem[mem_2]++;
            break;
        case oat_stairs_c:
            cond++;
            break;
        case southern_labs_1:
            mem[mem_1]--;
            break;
        case southern_labs_2:
            mem[mem_2]--;
            break;
        case southern_labs_c:
            cond--;
            break;

        case hall_13_1:
            mem[mem_1] = 0;
            break;
        case hall_13_2:
            mem[mem_2] = 0;
            break;
        case hall_13_3:
            mem[mem_3] = 0;
            break;
        case hall_13_c:
            cond = 0;
            break;

        case rm_1:
            mem_1++;
            break;
        case rm_2:
            mem_2++;
            break;
        case rm_3:
            mem_3++;
            break;
        case kd_1:
            mem_1--;
            break;
        case kd_2:
            mem_2--;
            break;
        case kd_3:
            mem_3--;
            break;
        case eshop_1:
            mem[mem_1]=mem[mem_1]*mem[mem_1];
            break;
        case eshop_2:
            mem[mem_2]=mem[mem_2]*mem[mem_2];
            break;
        default:
            cout << "No such operation exists" << endl;
    }
}

int traveller() {
    if (curr_lm == finish) return 0;

    if (curr_lm != start) operations(curr_lm);

    if (graph[curr_lm].find(cond) == graph[curr_lm].end()) {
        printf("Stuck in landmark number %d", curr_lm);
        cout << endl;
        return 1;
    }

    curr_lm = graph[curr_lm][cond];
    return traveller();
}

int main(int argc, char* argv[]) {
    if (argc != 2) {
        cout << "Wrong number of arguments\n";
        cout << "Usage: ./interpreter <filename>\n";
        return 1;
    }

    int exit_code = input(argv[1]);

    if (exit_code != 0) return exit_code;

    // for(int i = 0; i < lines.size(); i++){
    //     for(int j = 0; j < lines[i].size(); j++)
    //         cout << lines[i][j] << " ";
    //     cout << endl;
    // }

    exit_code = graph_builder();

    if (exit_code != 0) return exit_code;

    curr_lm = start;
    exit_code = traveller();

    if (exit_code != 0) return exit_code;

    return 0;
}