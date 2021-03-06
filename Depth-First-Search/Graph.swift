 protocol Graph{
    associatedtype ItemType:Hashable
    var pointSet:Set<ItemType> {set get}

    associatedtype PathType
    var pathDic:PathType {set get}
    init (V points:Set<ItemType>, E paths:PathType)
}

public class UndirGraph<U:Hashable>:Graph{
    var pointSet:Set<U>

    //typealias PathType = [U:Set<U>]
    var pathDic:[U:Set<U>]

    required public init(V points:Set<U>, E paths:[U:Set<U>]){
        self.pointSet = points
        self.pathDic = paths

        for p in pointSet{
            if self.pathDic[p] == nil {
                self.pathDic[p] = []
            }
        }
    }

    init(E paths:PathType) {
        self.pathDic = paths
        self.pointSet = []
        for (k, _) in paths {
            self.pointSet.insert(k)
        }
    }

    func addConnect(from s:U, _ pathSet:Set<U>) {
        guard self.pointSet.contains(s) else {
            print("\(s) is not the node in graph");return
        }

        let newPoint = pathSet.subtracting(pathDic[s]!) // store new points which need update

        var newPoint2add = Set<U>() // add set to store
        for p in newPoint {
            guard self.pointSet.contains(p) else {
                print("\(p) in pathSet is not the node in graph");return
            }
            newPoint2add.insert(p)
        }

        // update
        self.pathDic[s] = self.pathDic[s]!.union(pathSet) // update this point
        for p in newPoint2add{ // update other points
            self.pathDic[p]!.insert(s)
        }
    }
}
