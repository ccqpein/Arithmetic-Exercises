protocol Graph{
    var pointSet:Set<Int> {set get}
    var pathSet:Set<Set<Int>> {set get}
    init (points:Set<Int>, paths:Set<Set<Int>>)
}

class UndirGraph:Graph {
    var pointSet:Set
    var pathSet:Set<Set>
    init(){
        
    }
}
