(* just copy the lisp version*)

let number_to_list n : int list =
  let rec f n l = if n < 10 then n :: l else f (n / 10) ((n mod 10) :: l) in
  f n []

let list_numbers_join (nl : int list) =
  let ex = 10. ** float_of_int (List.length nl - 1) in
  let rec f nl ex : float =
    match nl with
    | n :: ns -> (float_of_int n *. ex) +. f ns (ex /. 10.)
    | [] -> 0.
  in
  f nl ex

let min_max_difference num : float =
  let a = number_to_list num in
  let pair = (ref 0, ref (List.nth a 0)) in
  let rec f nl =
    match nl with n :: ns -> if n != 9 then fst pair := n else f ns | [] -> ()
  in
  let rec collect_l nl result =
    match nl with
    | n :: ns ->
        if !(fst pair) = n then collect_l ns result @ [ 9 ]
        else collect_l ns result @ [ n ]
    | _ -> result
  in
  let rec collect_s nl result =
    match nl with
    | n :: ns ->
        if !(snd pair) = n then collect_s ns result @ [ 0 ]
        else collect_l ns result @ [ n ]
    | _ -> result
  in
  f a;
  list_numbers_join (collect_l a []) -. list_numbers_join (collect_s a [])
;;

min_max_difference 11891 = 99.;;
min_max_difference 90 = 99.
