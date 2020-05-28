function corpFlightBookings(bookings::Array{Array{Int64,1},1}, n::Int64)
    result::Array{Int64,1} = zeros(Int64,20000)
    for book in bookings
        for ind in book[1]:book[2] 
            result[ind] += book[3]
        end
    end
    result[1:n]
end
