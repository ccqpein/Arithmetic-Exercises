class Node{
    var value:Int
    var node:[Node]?
    var root:Node?
    var numberOfLevel = 0

    init(_ initvalue:Int){
        self.value = initvalue
    }
}

func connected(_ a:Node, _ b:Node){
    switch (a.root, b.root){
    case let (nil,nil):
        if a.numberOfLevel >= b.numberOfLevel{
            a.node.append(b)
        }else{
            b.node.append(a)
        }
    case let (x,nil):
        x.node.append(b)
    }
      
    
    a.node.append(b)
    a.numberOfNode += 1
    b.root = a
}
