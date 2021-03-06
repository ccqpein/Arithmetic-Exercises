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

struct Date {
    var d:Int
    var m:Int
    var y:Int
}

// Can use switch to make algorithm add date by year (now is by month)
// Todo: make this function have ability to decrease the date if input the num is negetive
func addDate(_ days:Int, _ input:Date) -> (reDate:Date, daysLeft:Int) {
    guard days > 0 else {
        print("sorry, days number is nagetive, try to use \"reduceDate\"")
        return (input, 0)
    }
    
    var monthTable:[Int] = leapYear(input.y) ? monthDayL : monthDay

    let sumDays:Int = days + input.d
    var reDate:Date = input
    var daysLeft:Int = 0
    
    if sumDays > monthTable[input.m - 1] {
        reDate.m += 1
        daysLeft = sumDays - monthTable[input.m - 1] - 1
        reDate.d = 1

        if reDate.m > 12 {
            reDate.y += 1
            reDate.m -= 12
        }
        
    } else {
        reDate.d = sumDays
    }

    return (reDate, daysLeft)
}


func reduceDate(_ days:Int, _ input:Date) -> (reDate:Date, daysLeft:Int) {
    guard days < 0 else {
        print("sorry, days number is positive, try to use \"addDate\"")
        return (input, 0)
    }
    
    var monthTable:[Int] = leapYear(input.y) ? monthDayL : monthDay

    let sumDays:Int = days + input.d //sumdays may negative 
    var reDate:Date = input
    var daysLeft = 0

    if sumDays > 0 {
        reDate.d = sumDays
    }else{
        reDate.m -= 1

        if reDate.m < 1 {
            reDate.y -= 1
            reDate.m  = 12
        }
        
        daysLeft = sumDays
        reDate.d = monthTable[reDate.m - 1]
    }

    return (reDate, daysLeft)
}


func main(_ days:Int, dateInput date:Date) -> Date {
    var daysLeft = days
    var reDate = date
    var f:((Int, Date) -> (Date, Int))?

    if days < 0 {
        f = reduceDate
    }else {
        f = addDate
    }
    
    while daysLeft != 0 {
        (reDate, daysLeft) = f!(daysLeft, reDate)
    }

    return reDate
    
}


let testa = Date(d:25, m:2, y:2004)
let testb = Date(d:25, m:12, y:2004)
let testc = Date(d:1, m:1, y:2005)

//print(main(370, dateInput:testa))
//print(main(7, dateInput:testb))

