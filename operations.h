#define start               0       // start
#define finish              1       // finish
#define iit_gate_in_1       2       // input to mem[mem_1]
#define iit_gate_in_2       3       // input to mem[mem_2]
#define hall_2              4       // mem[mem_3] = mem[mem_1] + mem[mem_2]
#define hall_3              5       // mem[mem_3] = mem[mem_1] * mem[mem_2]
#define hall_5              6       // mem[mem_3] = mem[mem_1] - mem[mem_2]
#define hall_12             7       // mem[mem_3] = mem[mem_1] / mem[mem_2]
#define mt_1_3              8       // mem[mem_1] = mem[mem_3]
#define mt_3_1              9       // mem[mem_3] = mem[mem_1]
#define mt_2_3              10      // mem[mem_2] = mem[mem_3]
#define mt_3_2              11      // mem[mem_3] = mem[mem_2]
#define iit_gate_out_1      12      // output mem[mem_1]
#define iit_gate_out_2      13      // output mem[mem_2]
#define lecture_hall_gt     14      // mem[mem_1] > mem[mem_2]
#define lecture_hall_gt_t   15
#define lecture_hall_gt_f   16     
#define lecture_hall_lt     17      // mem[mem_1] < mem[mem_2]
#define lecture_hall_lt_t   18
#define lecture_hall_lt_f   19
#define lecture_hall_eq     20      // mem[mem_1] == mem[mem_2]
#define lecture_hall_eq_t   21
#define lecture_hall_eq_f   22
#define oat_stairs_1        23      // mem[mem_1]++
#define oat_stairs_2        24      // mem[mem_2]++
#define oat_stairs_c        25      // cond++
#define southern_labs_1     26      // mem[mem_1]--
#define southern_labs_2     27      // mem[mem_2]--
#define southern_labs_c     28      // cond--
#define hall_13_1           29      // mem[mem_1] = 0
#define hall_13_2           30      // mem[mem_2] = 0
#define hall_13_3           31      // mem[mem_3] = 0
#define hall_13_c           32      // cond = 0
#define rm_1                33      // mem_1++
#define rm_2                34      // mem_2++
#define rm_3                35      // mem_3++
#define kd_1                36      // mem_1--
#define kd_2                37      // mem_2--
#define kd_3                38      // mem_3--
#define eshop_1             39      // mem[mem_1] = mem[mem_1]*mem[mem_1]
#define eshop_2             40      // mem[mem_2] = mem[mem_2]*mem[mem_2]
#define doaa_1              41      // output ascii char corresponding to mem[mem_1]
#define doaa_2              42      // output ascii char corresponding to mem[mem_2]

#define num_ops             43