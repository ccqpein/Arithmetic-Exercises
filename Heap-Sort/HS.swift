class Heap<T:Hashable & Comparable> {
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
        guard ind < self.value.count else {return (-1,nil)}
        let result = (ind, Optional.some(self[ind]))
        return result
    }
    func rightChild(_ index:Int) -> (Int,T?){
        let ind = 2*(index + 1)
        guard ind < self.value.count else {return (-1,nil)}
        let result = (ind, Optional.some(self[ind]))
        return result
    }
    
    func maxHeapify(_ index:Int) {
        guard index != -1 else {return}

        var thisV = (index,self[index])
        var leftC = self.leftChild(index)
        var rightC = self.rightChild(index)
        var nextIndex = -1

        switch (thisV,leftC,rightC) {
        case let ((i,x),(key,v),(ii,y)) where key == -1 && ii != -1:
            if x < y! {
                (self[i],self[ii]) = (y!,x)
                nextIndex = ii
            }else{
                break
            }
        case let ((i,x),(ii,y),(key,v)) where key == -1 && ii != -1:
            if x < y! {
                (self[i],self[ii]) = (y!,x)
                nextIndex = ii
            }else{
                break
            }
        case let ((i,x),(i1,y1),(i2,y2)) where i1 != i2: // if no children, i1 == i2 == -1
            if x > y1! && x > y2! {
                break
            }else if y1! > y2! {
                (self[i],self[i1]) = (y1!,x)
                nextIndex = i1
            }else {
                (self[i],self[i2]) = (y2!,x)
                nextIndex = i2
            }
            print(nextIndex)
        default:
            break
        }
        
        self.maxHeapify(nextIndex)
    }
}

var a = Heap([16,4,10,14,7,9,3,2,8,1])
