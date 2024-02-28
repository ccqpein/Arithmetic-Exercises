let reverse_str s =
  s |> String.to_seq |> List.of_seq |> List.rev |> List.to_seq |> String.of_seq
