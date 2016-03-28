package main

import (
	. "fmt"
)

type Node struct {
	ParentNode   *Node
	Data         int
	ChildrenNode [2]*Node
}

func (n *Node) insert(data int) {
	if data < n.Data {
		if n.ChildrenNode[0] != nil {
			n.ChildrenNode[0].insert(data)
		} else {
			nodeTemp := Node{Data: data, ParentNode: n}
			n.ChildrenNode[0] := *nodeTemp
		}
	} else if data > n.Data {
		if n.ChildrenNode[1] != nil {
			n.ChildrenNode[1].insert(data)
		} else {
			nodeTemp := Node{Data: data, ParentNode: n}
			n.ChildrenNode[1] := *nodeTemp
		}
	}
}

func main() {
	root := make(Node{Data: 8})

	root.insert(3)
	Println(root.Data)
	Println(root.ChildrenNode[0].Data)
}
