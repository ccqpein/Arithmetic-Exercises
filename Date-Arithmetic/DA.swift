let monthDay:[Int] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31,]
let monthDayS:[Int] = [0, 31, 59, 90, 120, 151, 181, 212, 243, 273, 304, 334, 365,]
let yzero = 2000

func leapYear(_ y:Int) -> Bool {
    if (y % 4 == 0) && (y % 400 == 0 || y % 100 != 0) {
        return true
    } else {
        return false
    }
}


