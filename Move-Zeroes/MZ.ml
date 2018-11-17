let mz =
  List.fold_right
    (fun x acc -> if x != 0
                  then x::acc
                  else List.append acc [x])
;;

let rec print_list = function
                   |[] -> ()
                   | e::l -> print_int e ; print_string " " ; print_list l
;;

print_list (mz [0;1;0;12;11] []);;
