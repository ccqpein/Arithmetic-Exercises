package main

import (
	"fmt"
	"time"
)

type ListNode struct {
	Val  int
	Next *ListNode
}

func handleSmall(lists []*ListNode) ([]*ListNode, *ListNode) {
	if len(lists) == 0 {
		return nil, nil
	}

	result := []*ListNode{}
	var smallest *ListNode
	ind := -1

	for idx, i := range lists {
		if i == nil {
			continue
		}
		if ind == -1 {
			ind = idx
			smallest = i
		}
		if i.Val < smallest.Val {
			smallest = i
			ind = idx
		}
		result = append(result, i)
	}

	if ind == -1 {
		return nil, nil
	}

	result[ind] = result[ind].Next

	return result, smallest
}

func mergeKLists(lists []*ListNode) *ListNode {
	if len(lists) == 0 {
		return nil
	}

	this_list := []*ListNode{}
	for _, i := range lists {
		if i != nil {
			this_list = append(this_list, i)
		}
	}

	var this_result *ListNode = &ListNode{}
	result := this_result

	for {
		new_l, smallest := handleSmall(this_list)
		temp_l := []*ListNode{}
		for _, i := range new_l {
			if i != nil {
				temp_l = append(temp_l, i)
			}
		}

		if smallest == nil {
			return nil
		}

		this_list = temp_l
		this_result.Val = smallest.Val
		if len(this_list) == 0 {
			break
		}

		this_result.Next = &ListNode{}
		//println(this_result.Val)
		this_result = this_result.Next
	}

	return result
}

func mergeKLists2(lists []*ListNode) *ListNode {
	var smallest *int
	smallest_ind := 0
	var result *ListNode

	for ind, i := range lists {
		if i != nil {
			if smallest == nil {
				smallest = &i.Val
				result = i
			}
			if i.Val <= *smallest {
				result = i
				smallest = &i.Val
				smallest_ind = ind
			}
		}
	}

	if result == nil {
		return nil
	} else {
		//fmt.Printf("%+v", lists[smallest_ind])
		lists[smallest_ind] = lists[smallest_ind].Next
		//fmt.Printf(" %+v\n", lists[smallest_ind])
		result.Next = mergeKLists2(lists)
		//println(result.Val)
		return result
	}
}

func mergeKLists3(lists []*ListNode) *ListNode {
	t0 := time.Now()
	var result *ListNode = &ListNode{}
	var mark *ListNode = result

	for {
		var smallest *int
		smallest_ind := 0
		for ind, i := range lists {
			if i != nil {
				if smallest == nil {
					smallest = &i.Val
				}
				if i.Val <= *smallest {
					smallest = &i.Val
					smallest_ind = ind
				}
			}
		}
		if smallest == nil {
			break
		}
		mark.Next = lists[smallest_ind]
		mark = mark.Next
		lists[smallest_ind] = lists[smallest_ind].Next

	}
	t1 := time.Now()
	fmt.Printf("cost is = %v\n", t1.Sub(t0).Nanoseconds())
	return result.Next
}

func printout(ln *ListNode) {
	if ln != nil {
		fmt.Printf("%+v -> ", ln.Val)
		printout(ln.Next)
	}
}

func main() {
	test0 := []*ListNode{
		&ListNode{
			Val: 1,
			Next: &ListNode{
				Val: 4,
				Next: &ListNode{
					Val:  5,
					Next: nil,
				},
			},
		},
		&ListNode{
			Val: 1,
			Next: &ListNode{
				Val: 3,
				Next: &ListNode{
					Val:  4,
					Next: nil,
				},
			},
		},
		&ListNode{
			Val: 2,
			Next: &ListNode{
				Val:  6,
				Next: nil,
			},
		},
	}
	// fmt.Printf("%+v\n", test0)
	// temp0, temp1 := handleSmall(test0)
	// fmt.Printf("%+v, %p\n", temp0, temp1)
	//fmt.Printf("%+v\n", mergeKLists(test0))
	printout(mergeKLists3(test0))
	//t0 := time.Now()
	//mergeKLists3(test0)
	//t1 := time.Now()
	//fmt.Printf("2 seconds is = %v\n", (2 * time.Second).Nanoseconds())

	test1 := []*ListNode{
		nil,
	}
	//fmt.Printf("%+v\n", mergeKLists(test1))
	fmt.Printf("%+v\n", mergeKLists2(test1))

	test2 := []*ListNode{
		nil,
		&ListNode{
			Val:  1,
			Next: nil,
		},
	}
	//fmt.Printf("%+v\n", mergeKLists(test2))
	fmt.Printf("%+v\n", mergeKLists2(test2))

	test3 := []*ListNode{
		&ListNode{
			Val: -3,
			Next: &ListNode{
				Val: 1,
				Next: &ListNode{
					Val:  4,
					Next: nil,
				},
			},
		},
		&ListNode{
			Val: -2,
			Next: &ListNode{
				Val: -1,
				Next: &ListNode{
					Val: 0,
					Next: &ListNode{
						Val:  2,
						Next: nil,
					},
				},
			},
		},
		&ListNode{
			Val: -1,
			Next: &ListNode{
				Val:  1,
				Next: nil,
			},
		},
	}

	//fmt.Printf("%+v\n", mergeKLists2(test3))
	printout(mergeKLists2(test3))
}
