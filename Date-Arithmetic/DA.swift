let monthDay:[Int] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,]
let monthDayL:[Int] = [31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,]
let monthDayS:[Int] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365,]
let yzero = 2000

func leapYear(_ y:Int) -> Bool {
    if (y % 4 == 0) && (y % 400 == 0 || y % 100 != 0) {
        return true
    } else {
        return false
    }
}

struct Date {
    var d:Int
    var m:Int
    var y:Int
}

// Can use switch to make algorithm add date by year (now is by month)
// Todo: make this function have ability to decrease the date if input the num is negetive
func addDate(_ days:Int, _ input:Date) -> (reDate:Date, daysLeft:Int) {
    var monthTable:[Int] = leapYear(input.y) ? monthDayL : monthDay
    
    var sumDays:Int = days + input.d
    var reDate = Date(d:0, m:0, y:0)
    var daysLeft:Int = 0

    if sumDays > monthTable[input.m] {
        reMon += 1
        daysLeft = sumDays - monthTable[input.m]
    }
}

func main(_ days:Int, dateInput date:Date) -> () {
    
}
