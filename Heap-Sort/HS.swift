class Heap<T:Hashable> {
    var value = Array<T>()

    init(_ l:[T]){
        self.value = l
    }

    subscript(index:Int) -> T {
        get {
            assert(index < self.value.count, "Index out of range")
            return self.value[index]
        }
        set {
            assert(index < self.value.count, "Index out of range")
            self.value[index] = newValue
        }
    }

    func parent(_ index:Int) -> (Int,T?){
        guard index < self.value.count else {print("index out of range");return (-1,nil)}
        guard index > 0 else {print("this is first node, no parent");return (-1,nil)}
        let ind = (index-1)/2
        let result = (ind, Optional.some(self[ind]))
        return result
    }
    
    func leftChild(_ index:Int) -> (Int,T?){
        let ind = 2*index + 1
        let result = (ind, Optional.some(self[ind]))
        return result
    }
    func rightChild(_ index:Int) -> (Int,T?){
        let ind = 2*(index + 1)
        let result = (ind, Optional.some(self[ind]))
        return result
    }
//    
//    mutating func maxHeapify() {
//        
//    }
}

var a = Heap([16,14,10,8,7,9,3,2,4,1])
