class PathGraph():
    pointSet = []
    pathLen = {}  # {point:{shortMark:Bool,p1:length,p2:length...},...}

    def __init__(self, points):
        self.pointSet = points

    def add_path(self, start, to, length):
        self.pathLen[start] = {to: length}
        self.pathLen[start]["shortest"] = False

    def path(self, start, to):
        try:
            return self.pathLen[start][to]
        except:
            print("No value")
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
