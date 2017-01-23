class Node{
    var value:Int
    var node = [Node]()
    var root:Node?
    var numberOfLevel = 0

    init(_ initvalue:Int){
        self.value = initvalue
        self.root = self
    }
}

func findRoot(_ n:Node) -> Node{
    guard n.root !== n else {return n}
    
    var result = findRoot(n.root!)

    return result
}

func connected(_ a:Node, _ b:Node){
    switch (findRoot(a), findRoot(b)) {
    case let (x,y) where x === a && y === b: // a and b are top nodes
        if a.numberOfLevel > b.numberOfLevel {
            a.node.append(b)
            b.root = a
        }else if a.numberOfLevel == b.numberOfLevel{
            a.node.append(b)
            b.root = a
            a.numberOfLevel += 1
        }else{
            b.node.append(a)
            a.root = b
        }
    case let (aroot,broot):
        if aroot.numberOfLevel > broot.numberOfLevel {
            aroot.node.append(broot)
            broot.root = aroot
        }else if aroot.numberOfLevel == broot.numberOfLevel{
            aroot.node.append(broot)
            broot.root = aroot
            aroot.numberOfLevel += 1
        }else{
            broot.node.append(aroot)
            aroot.root = broot
        }        
    }
}

func connected2(){ // function connected might be improved
}

// test cases

var n0 = Node(0)
var n1 = Node(1)
var n2 = Node(2)
var n3 = Node(3)
var n4 = Node(4)
var n5 = Node(5)
var n6 = Node(6)
var n7 = Node(7)
var n8 = Node(8)
var n9 = Node(9)

connected(n4,n3)
connected(n4,n8)
connected(n8,n9)

connected(n2,n1)
connected(n7,n1)

connected(n6,n0)
connected(n6,n5)
connected(n6,n2)

connected(n4,n0)

print(n0,n1,n2,n3,n4,n5,n6,n7,n8,n9)
