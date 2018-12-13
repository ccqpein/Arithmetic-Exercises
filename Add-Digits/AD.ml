let rec add_digits (num: int) : int =
  let sepa num = List.map (fun a -> Char.code a - 48)
                   (List.init
                      (String.length (string_of_int num))
                      (String.get (string_of_int num)))
  and sum :'int list -> int = List.fold_left (+) 0;
  in
  if num <0 then -1
  else if num >= 10 then add_digits (sum (sepa num))
  else num
;;
