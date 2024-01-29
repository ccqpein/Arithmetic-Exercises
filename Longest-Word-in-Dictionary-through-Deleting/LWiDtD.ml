let longest_string_sort (sl : string list) =
  List.sort (fun a b -> String.length b - String.length a) sl

let lexi_sort s1 s2 =
  let rec inner s1 s2 =
    match (s1, s2) with
    | x :: xs, y :: ys ->
        if x > y then 1 else if x == y then inner xs ys else -1
    | _ -> 0
  in
  inner (s1 |> String.to_seq |> List.of_seq) (s2 |> String.to_seq |> List.of_seq)

let rec check s word =
  match (s, word) with
  | _, [] -> true
  | [], _ -> false
  | s1 :: ss, w1 :: ws -> if s1 == w1 then check ss ws else check ss word

let find_longest_word s dicts =
  let string_to_list s = s |> String.to_seq |> List.of_seq in
  let sorted_dicts = longest_string_sort dicts in
  let rec helper flag last_len dicts re =
    match dicts with
    | [] -> re
    | word :: rest ->
        if String.length word < last_len && flag then re
        else if check (string_to_list s) (string_to_list word) then
          helper true (String.length word) rest (word :: re)
        else helper flag last_len rest re
  in
  List.nth (List.sort lexi_sort (helper false 0 sorted_dicts [])) 0
;;

"apple" = find_longest_word "abpcplea" [ "ale"; "apple"; "monkey"; "plea" ];;
"a" = find_longest_word "abpcplea" [ "a"; "b"; "c" ];;

"ewaf"
= find_longest_word "aewfafwafjlwajflwajflwafj"
    [ "apple"; "ewaf"; "awefawfwaf"; "awef"; "awefe"; "ewafeffewafewf" ]
