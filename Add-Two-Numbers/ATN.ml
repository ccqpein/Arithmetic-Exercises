(* use list instead create a new type *)
let rec add_two_numbers lx ly =
  check_ten (add_two_together lx ly)
;;

let rec add_two_together lx ly :int list =
  match lx,ly with
  |[],y -> y
  |x,[] -> x
  |x::xs,y::ys -> (x+y) :: add_two_together xs ys
;;

(* looks like last cannot be last var *)
let rec check_ten ?(last = 0) (l:int list) =
  match l with
  |[] -> if last = 0 then [] else [1]
  |x::xs -> if x+last  >= 10
           then (x+last-10) :: check_ten xs ~last:1
           else x+last :: check_ten xs ~last:0
;;
