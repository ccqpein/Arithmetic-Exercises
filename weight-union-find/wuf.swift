class Node{
    var value:Int
    var node:Node?

    init(_ initvalue:Int){
        self.value = initvalue
        self.node = nil
    }

    func addNode(_ otherNode:Node) {
        self.node = otherNode
    }
}

