class Graph():
    pointSet = []
    # it should be a dict on {point:[list of point it connects]}
    pathDic = dict()
    pointTime = dict()  # points' discover time and finish time

    def __init__(self, points, paths):
        if set(paths.keys()) - set(points):
            print(
                "Warning: Some points in path dict not exist in points set,\
 initialize fail!")
            return

        self.pointSet = points
        self.pointSet.sort()  # sort points to alphabetical
        self.pathDic = paths
        # init each points discovery and finishing time list
        self.pointTime = {key: [] for key in self.pointSet}

        for i in self.pointSet:
            try:
                self.pathDic[i].sort()  # sort paths list to alphabetical
            except KeyError:  # if some point has no path, give it a empty list
                self.pathDic[i] = []


def DFS(g):
    time = 0
    greyMark = set()  # grey set for storing all points in recursive
    blackMark = set()  # black set for storing all points have done

    def DFS_inner(g, i, time):  # recursive function
        time += 1
        greyMark.add(i)  # add to grey set
        g.pointTime[i].append(time)  # store discover time
        for c in g.pathDic[i]:
            if c in blackMark or c in greyMark:
                pass
            else:
                time = DFS_inner(g, c, time)
        time += 1
        # store finish time, so finish time's index is 1
        g.pointTime[i].append(time)
        blackMark.add(i)  # finish
        greyMark.remove(i)  # delete grey set
        return time

    for i in g.pointSet:
        if i in blackMark or i in greyMark:
            pass
        else:
            time = DFS_inner(g, i, time)

    # format print
    for k in g.pointSet:
        print("{0} -> discover time is {1} -> finish time is {2}"
              .format(k, g.pointTime[k][0], g.pointTime[k][1]))
    return "done"


def topologicalSort(g):
    DFS(g)  # in case graph has not DFSed before

    # create list of turtle that [(point, finish time)]
    finishTimeList = []
    for k, v in g.pointTime.items():
        finishTimeList.append((k, v[1]))  # v[1] is finish time

    # sort elements increasing by finish time
    finishTimeList.sort(key=lambda pair: pair[1])

    # insert on the front of result list
    result = []
    for i in finishTimeList:
        result.insert(0, i[0])

    # format print
    reStr = result[0]
    for i in result[1:]:
        reStr += " -> " + i
    print(reStr)
    return "done"

if __name__ == "__main__":
    # test
    test1set = ["u", "v", "w", "x", "y", "z", ]
    test1path = {"u": ["x", "v"],
                 "v": ["y"],
                 "w": ["y", "z"],
                 "x": ["v"],
                 "y": ["x"],
                 "z": ["z"],
                 }

    # q1 data
    q1set = ["s", "v", "w", "q", "t", "x", "z", "y", "r", "u", ]
    q1path = {"s": ["v"],
              "v": ["w"],
              "w": ["s"],
              "q": ["s", "w", "t"],
              "t": ["x", "y"],
              "x": ["z"],
              "z": ["x"],
              "y": ["q"],
              "r": ["u", "y"],
              "u": ["y"],
              }

    # q2 data
    q2set = ["m", "n", "q", "o", "p", "r", "s",
             "t", "u", "v", "w", "x", "y", "z", ]
    q2path = {"m": ["x", "q", "r", ],
              "n": ["o", "q", "u", ],
              "q": ["t", ],
              "o": ["r", "s", "v", ],
              "p": ["o", "s", "z", ],
              "r": ["u", "y", ],
              "s": ["r", ],
              "t": [],
              "u": ["t", ],
              "v": ["x", "w", ],
              "w": ["z", ],
              "x": [],
              "y": ["v", ],
              "z": [],
              }

    # test1 = Graph(test1set, test1path)
    q1 = Graph(q1set, q1path)
    q2 = Graph(q2set, q2path)

    DFS(q1)
    print("\n")
    topologicalSort(q2)
    print("\n")
