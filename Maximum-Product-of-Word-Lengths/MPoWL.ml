(* whooo *)
let string_to_chars s : char list = s |> String.to_seq |> List.of_seq
let string_to_chars_seq s = s |> String.to_seq

module CharSet = Set.Make (Char)

(* make sets of words *)
let rec make_char_set words =
  match words with
  | word :: ws ->
      let cs = CharSet.of_list (string_to_chars word) in
      cs :: make_char_set ws
  | [] -> []

let max_product words =
  let words_sets = make_char_set words in
  let most = ref 0 in
  for i = 0 to List.length words_sets - 1 do
    for j = i + 1 to List.length words_sets - 1 do
      if
        CharSet.is_empty
          (CharSet.inter (List.nth words_sets i) (List.nth words_sets j))
      then
        most :=
          max !most
            ((List.nth words_sets i |> CharSet.to_list |> List.length)
            * (List.nth words_sets j |> CharSet.to_list |> List.length))
    done
  done;
  !most
;;

max_product [ "abcw"; "baz"; "foo"; "bar"; "xtfn"; "abcdef" ] = 16;;
max_product [ "a"; "ab"; "abc"; "d"; "cd"; "bcd"; "abcd" ] = 4
