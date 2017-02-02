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
        self.pointSet.sort()
        self.pathDic = paths

        for i in self.pointSet:
            try:
                self.pathDic[i].sort()  # sort paths list to alphabetical
            except KeyError:  # if some point has no path, give it a empty list
                self.pathDic[i] = []


# data test
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

q1 = Graph(q1set, q1path)


def DFS(g):
