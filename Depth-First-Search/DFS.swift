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
                       "a":Set(["c","d","s"]),
                       "d":Set(["a"]),
                       "c":Set(["a"])])

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
        print(s)
        guard !stack.isEmpty else {return}
        var root = s
        doneNode.insert(s)
        var nodes = g.pathDic[s]

        if nodes!.subtracting(doneNode).isEmpty {
            stack.removeLast()
        }else{
            var next = nodes!.subtracting(doneNode).first!
            stack.append(next)
        }
        if !stack.isEmpty{
            innerDFS(g:g, s:stack.last!)
        }
    }
    
    innerDFS(g:g,s:s)
}
