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

    init(days:Int, month:Int, year:Int) {
        self.d = days
        self.m = month
        self.y = year
    }
    
    func addDate(days:Int) {

        var daysLeft:Int = days + self.d
        
        while daysLeft > 0 {
            var monthTable:[Int] = leapYear(self.y) ? monthDayL : monthDay

            if daysLeft > monthTable[self.m - 1] {
                daysLeft = daysLeft - monthTable[self.m - 1]
                self.m += 1
                self.d = 1
                
                if self.m > 12 {
                    self.y += 1
                    self.m -= 12
                }
                
            } else {
                self.d = daysLeft
                daysLeft = 0
            }
        }
    }

    func reduceDate(days:Int) {

        var daysLeft:Int = self.d - days

        while daysLeft <= 0 {
            var monthTable:[Int] = leapYear(self.y) ? monthDayL : monthDay

            self.m -= 1
            if self.m < 1 {
                self.y -= 1
                self.m = 12
            }
            
            daysLeft += monthTable[self.m - 1]
        }

        self.d = daysLeft

    }
}

var testa = Date(days:25, month:2, year:2004)
testa.addDate(days:370)
testa.reduceDate(days:370)
