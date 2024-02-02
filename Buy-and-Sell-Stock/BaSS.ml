let rec max_profit (prices : int list) : int =
  let rec inner max min result restL =
    match restL with
    | this :: xs ->
        if this < min then inner max this (if result < 0 then 0 else result) xs
        else if this - min > result then
          inner this min (if this - min < 0 then 0 else this - min) xs
        else inner max min (if result < 0 then 0 else result) xs
    | [] -> if result < 0 then 0 else result
  in
  match prices with x :: xs -> inner 0 x 0 xs | [] -> 0
;;

max_profit [] == 0;;
max_profit [ 2; 1; 2; 1; 0; 1; 2 ] == 2;;
max_profit [ 1 ] == 0;;
max_profit [ 7; 1; 5; 3; 6; 4 ] == 5;;
max_profit [ 2; 4; 1 ] == 2
