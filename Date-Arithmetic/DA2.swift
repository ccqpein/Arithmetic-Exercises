// Use different design

let monthDay:[Int] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,]
let monthDayL:[Int] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,]
let yzero = 2000

func leapYear(_ y:Int) -> Bool {
    if (y % 4 == 0) && (y % 400 == 0 || y % 100 != 0) {
        return true
    } else {
        return false
    }
}

class Date {
    var d:Int
    var m:Int
    var y:Int

    func addDate(_ days:Int) -> 
    
}
