(* use list instead create a new type *)

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

let rec add_two_numbers lx ly =
  check_ten (add_two_together lx ly)
;;

let rec print_list = function
  |[] -> ()
  | e::l -> print_int e ; print_string " " ; print_list l
;;

print_list (add_two_together [3;4;5] [1;2]);;

(* use node list *)
type list_node = Empty | Node of int * list_node;;

let rec list_node_to_string =function
  |Empty -> ";"
  |Node(v,nextn) -> Printf.sprintf "%d " v ^ list_node_to_string nextn
;;

exception SomethingWrong;;

let rec add_two_together_nodes nx ny:list_node =
  match nx,ny with
  |Empty,Empty -> Empty
  |Empty,Node(_) -> ny
  |Node(_), Empty -> nx
  |Node(vx,nextx), Node(vy,nexty) -> Node(vx+vy, add_two_together_nodes nextx nexty)
;;

let rec check_ten_nodes ?(last = 0) (l:list_node) =
  match l with
  |Empty -> if last = 1 then Node(1,Empty) else Empty
  |Node(v,nextn) -> if v + last >9
                    then Node(v+last-10, check_ten_nodes ~last:1 nextn)
                    else Node(v+last, check_ten_nodes nextn)
;;

let rec add_two_numbers_nodes lx ly =
  check_ten_nodes (add_two_together_nodes lx ly)
;;

let a1 = Node(1, Empty);;
let a2 = Node(9,Node(2,Empty));;

let b1 = Node(2,Empty);;
let b2 = Node(8,Node(9,Node(9,Empty)));;

let c1 = Node(5,Empty);;
let c2 = Node(5,Empty);;


list_node_to_string (add_two_numbers_nodes a1 a2);;
list_node_to_string (add_two_numbers_nodes b1 b2);;
list_node_to_string (add_two_numbers_nodes c1 c2);;

