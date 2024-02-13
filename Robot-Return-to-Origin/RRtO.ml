let judge_circle moves =
  let h = ref 0 and c = ref 0 in
  moves |> String.to_seq |> List.of_seq
  |> List.iter (fun cc ->
         match cc with
         | 'U' -> h := !h + 1
         | 'D' -> h := !h - 1
         | 'L' -> c := !c - 1
         | 'R' -> c := !c + 1
         | _ -> ());
  !h == 0 && !c == 0

let main =
  assert (judge_circle "UD");
  assert (not (judge_circle "LL"))
