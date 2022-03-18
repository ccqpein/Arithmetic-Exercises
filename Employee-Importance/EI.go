package main

type Employee struct {
	Id           int
	Importance   int
	Subordinates []int
}

func getImportance_bkp(employees []*Employee, id int) int {
	for _, e := range employees {
		if e.Id == id {
			next := 0
			for _, sub := range e.Subordinates {
				next += getImportance_bkp(employees, sub)
			}
			return next + e.Importance
		}

	}

	return 0
}

// even slower
func getImportance(employees []*Employee, id int) int {
	table := map[int]*Employee{}
	for _, e := range employees {
		table[e.Id] = e
	}

	var inner func(ee map[int]*Employee, id int) int
	inner = func(ee map[int]*Employee, id int) int {
		result := ee[id].Importance
		for _, nextId := range ee[id].Subordinates {
			result += inner(ee, nextId)
		}
		return result
	}

	return inner(table, id)
}

func main() {}
