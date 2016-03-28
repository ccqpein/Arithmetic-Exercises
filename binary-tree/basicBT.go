package main

import (
	. "fmt"
)

type Node struct {
	ParentNode   *Node
	Data         int
	ChildrenNode [2]*Node
}

func (n *Node) Insert(data int) {
	nodeTemp := Node{Data: data, ParentNode: n}
	if data < n.Data {
		if n.ChildrenNode[0] != nil {
			n.ChildrenNode[0].Insert(data)
		} else {
			n.ChildrenNode[0] = &nodeTemp
		}
	} else if data > n.Data {
		if n.ChildrenNode[1] != nil {
			n.ChildrenNode[1].Insert(data)
		} else {
			n.ChildrenNode[1] = &nodeTemp
		}
	}
}

func (n *Node) Lookup(data int) (*Node, *Node) {
	var rn, p *Node
	if data < n.Data {
		if n.ChildrenNode[0] == nil {
			rn = nil
			p = n
		} else {
			rn, p = n.ChildrenNode[0].Lookup(data)
		}
	} else if data > n.Data {
		if n.ChildrenNode[1] == nil {
			rn = nil
			p = n
		} else {
			rn, p = n.ChildrenNode[1].Lookup(data)
		}
	} else {
		rn = n
		p = n.ParentNode
	}
	return rn, p
}

func main() {
	root := Node{Data: 8}

	root.Insert(3)
	root.Insert(10)
	root.Insert(1)
	root.Insert(6)
	root.Insert(4)
	root.Insert(7)
	root.Insert(14)
	root.Insert(13)

	Println(root.Data)
	Println(root.ChildrenNode[0])
	Println(root.Lookup(4))
}
