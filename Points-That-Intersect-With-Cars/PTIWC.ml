let number_of_points (nums : (int * int) list) =
  let snums = List.sort (fun x y -> fst x - fst y) nums in
  let rec inner x y rest result =
    (* Printf.printf "%d\n" x; *)
    (* Printf.printf "%d\n" y; *)
    match rest with
    | [] -> result
    | (a, b) :: xs ->
        if a <= y then
          if b >= y then inner x b xs (result + b - y) else inner x y xs result
        else inner x b xs (1 + result + b - a)
  in
  match snums with (x, y) :: rest -> inner x y rest (y - x + 1) | _ -> 0

let main =
  assert (number_of_points [ (3, 6); (1, 5); (4, 7) ] = 7);
  assert (number_of_points [ (1, 3); (5, 8) ] = 7);
  assert (number_of_points [ (2, 3); (3, 9); (5, 7); (4, 10); (9, 10) ] = 9)
