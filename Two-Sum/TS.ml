exception Empty_list;;
exception Nothing;;

let rec num_sum nums target ind=
  let rec inner_func num rest target ind=
    match rest with
    |[] -> raise Nothing;
    | x::xs -> if num +  x  = target
               then ind
               else inner_func num xs target (ind + 1)
  in
  match nums with
  |[] -> raise Empty_list
  |x::xs -> try (ind, inner_func x xs target (ind+1))
            with Nothing ->
              num_sum xs target (ind+1)
;;

num_sum [2;7;11;15] 9 0;;
num_sum [0;4;3;0] 0 0;;
num_sum [-3;4;3;90] 0 0;;
