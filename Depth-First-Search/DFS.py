class Graph():
    pointSet = []
    # it should be a dict on {point:[list of point it connect]}
    pathDic = dict()
    pointTime = dict()

    def __init__(self, points, paths):
        if set(paths.keys()) - set(points):
            print(
                "Warning: Some points in path dict not exist in points set,\
 initialize fail!")
            return

        self.pointSet = points
        self.pointSet.sort()  # sort points to alphabetical
        self.pathDic = paths

        for i in self.pointSet:
            try:
                self.pathDic[i].sort()  # sort paths list to alphabetical
            except KeyError:  # if some point has no path, give it a empty list
                self.pathDic[i] = []

            # init each points discovery and finishing time list
            self.pointTime[i] = []

# test
test1set = ["u", "v", "w", "x", "y", "z", ]
test1path = {"u": ["x", "v"],
             "v": ["y"],
             "w": ["y", "z"],
             "x": ["v"],
             "y": ["x"],
             "z": ["z"],
             }

# test1 = Graph(test1set, test1path)


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

# q1 = Graph(q1set, q1path)


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

# q2 = Graph(q2set,q2path)


def DFS(g):
    time = 0
    greyMark = set()
    blackMark = set()

    def DFS_inner(g, i, time):
        time += 1
        greyMark.add(i)
        g.pointTime[i].append(time)
        for c in g.pathDic[i]:
            if c in blackMark or c in greyMark:
                pass
            else:
                time = DFS_inner(g, c, time)
        time += 1
        g.pointTime[i].append(time)
        blackMark.add(i)
        greyMark.remove(i)
        return time

    for i in g.pointSet:
        if i in blackMark or i in greyMark:
            pass
        else:
            time = DFS_inner(g, i, time)

    return g.pointTime


def topologicalSort(g):
    DFS(g)  # in case graph has not DFSed before
    finishTimeList = []
    for k, v in g.pointTime.items():
        finishTimeList.append((k, v[1]))

    finishTimeList.sort(key=lambda pair: pair[1])

    result = []
    for i in finishTimeList:
        result.insert(0, i[0])

    return result
