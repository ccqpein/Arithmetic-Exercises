class Node{
    var value:Int
    var node:[Node]?
    var root:Node?
    var numberOfNode = 0

    init(_ initvalue:Int){
        self.value = initvalue
    }
}

func connected(_ a:Node, _ b:Node){
    a.node.append(b)
    a.numberOfNode += 1
    b.root = a
}
