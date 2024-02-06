let rec change_coin value coinList table : int =
  (* Printf.printf "get call value: %d\n" value; *)
  let rec loop_list value coinL orginal_coinL table =
    (* Printf.printf "loop: %d\n" value; *)
    (* print_string (String.concat " " (List.map string_of_int coinL)); *)
    (* print_string "\n"; *)
    match coinL with
    | c :: cs ->
        if value > c then
          (1 + change_coin (value - c) orginal_coinL table)
          :: loop_list value cs orginal_coinL table
        else loop_list value cs orginal_coinL table
    | [] -> []
  in
  if Hashtbl.mem table value then Hashtbl.find table value
  else
    let temp = List.sort compare (loop_list value coinList coinList table) in
    (* print_string (String.concat " " (List.map string_of_int temp)); *)
    (* print_string "\n"; *)
    (* Printf.printf "value: %d\n" value; *)
    if List.length temp > 0 then Hashtbl.add table value (List.nth temp 0);
    Hashtbl.find table value

let main =
  let table = Hashtbl.create 99 in
  List.iter (fun x -> Hashtbl.add table x 1) [ 1; 5; 10; 25 ];
  Printf.printf "%B\n" (change_coin 100 [ 1; 5; 10; 25 ] table = 4);
  Printf.printf "%B\n" (change_coin 1030 [ 1; 5; 10; 25 ] table = 42);
  Printf.printf "%B\n" (change_coin 302 [ 1; 5; 10; 25 ] table = 14)
