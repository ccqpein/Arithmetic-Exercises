protocol Graph{
    associatedtype ItemType:Hashable
    var pointSet:Set<ItemType> {set get}

    associatedtype PathType
    var pathDic:PathType {set get}
    init (V points:Set<ItemType>, E paths:PathType)
}

class UndirGraph<U:Hashable>:Graph{
    var pointSet:Set<U>

    typealias PathType = [U:Set<U>]
    var pathDic:PathType

    required init(V points:Set<U>, E paths:PathType){
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
        for (k, p) in paths {
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


//class DirGraph

var g = UndirGraph(V:Set([1,2,3,4,5]),
                   E:[1:Set([2,5]),
                      2:Set([1,3,4,5]),
                      3:Set([2,4]),
                      4:Set([2,3,5]),
                      5:Set([1,2,4])])

// Second way to create the graph
//var gg = UndirGraph(E:[1:Set([2,5]),
//                       2:Set([1,3,4,5]),
//                       3:Set([2,4]),
//                       4:Set([2,3,5]),
//                       5:Set([1,2,4])]) 

var ga = UndirGraph(V:Set(["s","a","b","c","d"]),
                    E:["s":Set(["a","b"]),
                       "a":Set(["c","d","s"])])

func BFS<T:Hashable>(graph g:UndirGraph<T>, start s:T) {
    var doneNode:Set<T> = [s]
    var nextNodes:Set<T> = [s]

    while(!nextNodes.isEmpty) {
        var tempNext:Set<T> = []
        let thisNode = nextNodes
        doneNode = doneNode.union(thisNode)
        
        for n in thisNode{
            print("this node is \(n)")
            tempNext = tempNext.union(g.pathDic[n]!.subtracting(doneNode))
        }
        nextNodes = tempNext
    }
}

func DFS<T:Hashable>(graph g:UndirGraph<T>, start s:T) {
    var stack = [s]
    var doneNode:Set<T> = []

    func innerDFS(g:UndirGraph<T>, s:T){
        var root = s
        var nodes = g.pathDic[s]
        if !nodes!.subtracting(doneNode).isEmpty {
            
        }else{
            
        }
    }
}
