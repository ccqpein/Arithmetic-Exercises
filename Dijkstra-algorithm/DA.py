class PathGraph():
    pointSet = []
    # {point:{shortMark:Bool,val:{targetPoint1:length,p2:length,..},..}}
    pathLen = {}
    pathList = {}  # {start:{end:[points],..},..}

    def __init__(self, points):
        self.pointSet = points
        for p in self.pointSet:
            self.pathLen[p] = {"shortMark": False, "val": {p: 0}}
            self.pathList[p] = {p: [p]}

    def add_path(self, start, to, length):
        self.pathLen[start]["val"][to] = length
        self.pathLen[start]["shortest"] = False

        self.pathList[start][to] = [start, to]

    def path(self, start, to=None):
        if not to:
            return self.pathLen[start]["val"]

        try:
            return self.pathLen[start]["val"][to]
        except:
            # print("No value")
            return None

    def shortest(self, start, to):
        tempVar = self.path(start, to)
        if tempVar is not None:
            if self.pathLen[start]["shortest"]:
                print("{0}".format(tempVar))
                return tempVar
            else:
                print("value is {0}, but not sure shortest or not"
                      .format(tempVar))
                return tempVar
        else:
            print("No value")
            return None


def Dijkstra(g, start):
    tempSet = set(g.pointSet)

    while len(tempSet) != 0:
        for v in sorted(g.path(start).items(), key=lambda x: x[1]):
            if v[0] in tempSet:
                thisPoint = v[0]
                break
        # print(thisPoint)
        for (k, v) in g.path(thisPoint).items():
            sumValue = g.path(start, thisPoint) + g.path(thisPoint, k)
            try:
                if sumValue < g.path(start, k):
                    g.path(start)[k] = sumValue
                    g.pathList[start][k] = g.pathList[start][thisPoint] + [k]
            except:
                g.add_path(start, k, sumValue)
                g.pathList[start][k] = g.pathList[start][thisPoint] + [k]
        tempSet.remove(thisPoint)

if __name__ == "__main__":
    qset = ["s", "t", "y", "x", "z"]

    a = PathGraph(qset)
    a.add_path("s", "t", 10)
    a.add_path("s", "y", 5)
    a.add_path("t", "y", 2)
    a.add_path("t", "x", 1)
    a.add_path("x", "z", 4)
    a.add_path("y", "t", 3)
    a.add_path("y", "x", 9)
    a.add_path("y", "z", 2)
    a.add_path("z", "x", 6)
    a.add_path("z", "s", 7)
    Dijkstra(a, "s")
    print(a.path("s"))
    print(a.pathList["s"])

    a = PathGraph(qset)
    a.add_path("s", "t", 10)
    a.add_path("s", "y", 5)
    a.add_path("t", "y", 2)
    a.add_path("t", "x", 1)
    a.add_path("x", "z", 4)
    a.add_path("y", "t", 3)
    a.add_path("y", "x", 9)
    a.add_path("y", "z", 2)
    a.add_path("z", "x", 6)
    a.add_path("z", "s", 7)

    Dijkstra(a, "z")
    print(a.path("z"))
    print(a.pathList["z"])
