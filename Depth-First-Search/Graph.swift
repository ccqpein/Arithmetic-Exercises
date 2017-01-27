protocol Graph{
    associatedtype ItemType:Hashable
    var pointSet:Set<ItemType> {set get}

    associatedtype PathType 
    var pathSet:PathType {set get}
    init (V points:Set<ItemType>, E paths:PathType)
}

class UndirGraph<U:Hashable>:Graph{
    var pointSet:Set<U>

    typealias PathType = Set<Set<U>>
    var pathSet:PathType

    required init(V points:Set<U>, E paths:PathType){
        self.pointSet = points
        self.pathSet = paths
    }
}
