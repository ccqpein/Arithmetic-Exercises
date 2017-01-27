protocol Graph{
    associatedtype ItemType:Hashable
    var pointSet:ItemType {set get}
    var pathSet:Set<ItemType> {set get}
    init (V points:ItemType, E paths:Set<ItemType>)
}

class UndirGraph:Graph {
    typealias ItemType = Set<Int>

    var pointSet:Set<Int>
    var pathSet:Set<Set<Int>>

    required init(V points:Set<Int>, E paths:Set<Set<Int>>){
        self.pointSet = points
        self.pathSet = paths
    }
}
