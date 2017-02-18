# 1--car,2--goat,3--goat
# 1--get reward, 0--no reward
import random


def pick_one(doorNum, switch=False):
    if switch:
        if doorNum == 1:
            return 0
        else:
            return 1
    else:
        if doorNum == 1:
            return 1
        else:
            return 0

if __name__ == "__main__":
    print("total: 100000")
    pick_num_list = [random.randint(1, 3) for i in range(100000)]
    switch_time, switch_win = 0, 0
    stay_time, stay_win = 0, 0
    for i in pick_num_list:
        if random.randint(0, 1) == 1:
            switch_time += 1
            if pick_one(i, switch=True) == 1:
                switch_win += 1
        else:
            stay_time += 1
            if pick_one(i) == 1:
                stay_win += 1

    print('''switch: {0}
switch win: {1} {2:.0%}
stay: {3}
stay_win:{4} {5:.0%}'''
          .format(switch_time, switch_win, switch_win / switch_time,
                  stay_time, stay_win, stay_win / stay_time)
          )
