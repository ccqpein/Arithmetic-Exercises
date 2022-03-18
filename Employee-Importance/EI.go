package main

type Employee struct {
	Id           int
	Importance   int
	Subordinates []int
}

func getImportance(employees []*Employee, id int) int {
	for _, e := range employees {
		if e.Id == id {
			next := 0
			for _, sub := range e.Subordinates {
				next += getImportance(employees, sub)
			}
			return next + e.Importance
		}

	}

	return 0
}

func main() {}
