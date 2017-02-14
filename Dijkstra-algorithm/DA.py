#! /usr/bin/env python3


class PathGraph():
    pointSet = []  # store all points

    # structure:
    # {point:{shortMark:Bool,val:{targetPoint1:length,targetPoint2:length,..},..}}
    pathLen = {}
    # store all path {start:{end:[points],..},..}
    pathList = {}

    def __init__(self, points):
        self.pointSet = points
        for p in self.pointSet:
            self.pathLen[p] = {"shortMark": False, "val": {p: 0}}
            self.pathList[p] = {p: [p]}  # the path that point to itself

    def add_path(self, start, to, length):
        self.pathLen[start]["val"][to] = length

        # make Mark false after every change
        self.pathLen[start]["shortest"] = False

        self.pathList[start][to] = [start, to]

    def path(self, start, to=None):
        # return the value from point "start" to point "to"
        if not to:
            return self.pathLen[start]["val"]

        try:
            return self.pathLen[start]["val"][to]
        except:
            # print("No value")
            return None

    def shortest(self, start, to):
        # return answer that whether all paths start from this point are
        # optimum
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
        # pick shortest point from tempSet
        for v in sorted(g.path(start).items(), key=lambda x: x[1]):  # sort
            if v[0] in tempSet:  # v[0] is smallest value
                thisPoint = v[0]
                break
        # print(thisPoint)
        for (k, v) in g.path(thisPoint).items():
            sumValue = g.path(start, thisPoint) + g.path(thisPoint, k)
            try:  # because g.path(start,k) might be None
                if sumValue < g.path(start, k):
                    g.path(start)[k] = sumValue  # update val dictionary
                    g.pathList[start][k] = g.pathList[start][
                        thisPoint] + [k]  # update path list
            except:
                # if none value, add to dictionary
                g.add_path(start, k, sumValue)
                g.pathList[start][k] = g.pathList[start][
                    thisPoint] + [k]  # update path list
        tempSet.remove(thisPoint)

if __name__ == "__main__":
    qset = ["s", "t", "y", "x", "z"]

    print("First from \"s\"")
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

    # format print
    for (k, v) in a.path("s").items():
        print("s -> {0} is {1}".format(k, v))

    for (k, v) in a.pathList["s"].items():
        print("s -> {0} path is {1}".format(k, v))

    print("\nNow from \"z\"")
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

    # format print
    for (k, v) in a.path("z").items():
        print("z -> {0} is {1}".format(k, v))

    for (k, v) in a.pathList["z"].items():
        print("z -> {0} path is {1}".format(k, v))
