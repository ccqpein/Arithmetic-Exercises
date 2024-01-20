(* just copy the lisp version*)

let number_to_list n : int list =
  let rec f n l = if n < 10 then n :: l else f (n / 10) ((n mod 10) :: l) in
  f n []

let list_numbers_join (nl : int list) =
  let ex = 10. ** float_of_int (List.length nl - 1) in
  let f nl ex : float =
    match nl with
    | n :: ns -> (float_of_int n * ex) + f ns (ex / 10)
    | n :: [] -> float_of_int n * ex
  in
  f nl ex
