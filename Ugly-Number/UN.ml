open List;;
open Float;;

let rec isUgly (n:int) :bool =
  let member_func i = exists (fun (x) -> x = i)
  in
  if member_func n [1;2;3;5] then true
  else if (mod_float (of_int n) 2.) = 0. then isUgly (n/2)
  else if (mod_float (of_int n) 3.) = 0. then isUgly (n/3)
  else if (mod_float (of_int n) 5.) = 0. then isUgly (n/5)
  else false
;;
