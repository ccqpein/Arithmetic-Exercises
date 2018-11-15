let mz =
  List.fold_right (fun x acc -> if x != 0
                                then x::acc
                                else List.append acc [x])
;;
