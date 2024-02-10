module NodeSet = Set.Make (Int)

let eadges_in_table table edges =
  List.iter
    (fun v ->
      let a = List.nth v 0 and b = List.nth v 1 in
      (match Hashtbl.find_opt table a with
      | Some l -> Hashtbl.replace table a (b :: l)
      | None -> Hashtbl.add table a [ b ]);
      match Hashtbl.find_opt table b with
      | Some l -> Hashtbl.replace table b (a :: l)
      | None -> Hashtbl.add table b [ a ])
    edges

let valid_path steps edges source dest =
  let rec helper (table : (int, int list) Hashtbl.t)
      (cache : (int, bool) Hashtbl.t) (paths : NodeSet.t ref) this endd length =
    if Hashtbl.mem cache this then false
    else if this = endd then true
    else
      (paths := NodeSet.add this !paths;
       List.find_opt
         (fun v ->
           if v = endd then true
           else if length = 0 then (
             paths := NodeSet.remove this !paths;
             false)
           else if NodeSet.mem v !paths then (
             paths := NodeSet.remove this !paths;
             false)
           else
             let x = helper table cache paths v endd (length - 1) in
             Hashtbl.add cache this x;
             x)
         (Hashtbl.find table this))
      |> Option.is_some
  in

  let table = Hashtbl.create 10000
  and paths = ref NodeSet.empty
  and cache = Hashtbl.create 10000 in

  eadges_in_table table edges;
  helper table cache paths source dest steps

let main =
  assert (valid_path 3 [ [ 0; 1 ]; [ 1; 2 ]; [ 2; 0 ]; [ 0; 2 ] ] 0 2);
  assert (
    not (valid_path 6 [ [ 0; 1 ]; [ 0; 2 ]; [ 3; 5 ]; [ 5; 4 ]; [ 4; 3 ] ] 0 5));
  assert (valid_path 1 [] 0 0);
  assert (
    not
      (valid_path 50
         [
           [ 31; 5 ];
           [ 10; 46 ];
           [ 19; 31 ];
           [ 5; 1 ];
           [ 31; 28 ];
           [ 28; 29 ];
           [ 8; 26 ];
           [ 13; 23 ];
           [ 16; 34 ];
           [ 30; 1 ];
           [ 16; 18 ];
           [ 33; 46 ];
           [ 27; 35 ];
           [ 2; 25 ];
           [ 49; 33 ];
           [ 44; 19 ];
           [ 22; 26 ];
           [ 30; 13 ];
           [ 27; 12 ];
           [ 8; 16 ];
           [ 42; 13 ];
           [ 18; 3 ];
           [ 21; 20 ];
           [ 2; 17 ];
           [ 5; 48 ];
           [ 41; 37 ];
           [ 39; 37 ];
           [ 2; 11 ];
           [ 20; 26 ];
           [ 19; 43 ];
           [ 45; 7 ];
           [ 0; 21 ];
           [ 44; 23 ];
           [ 2; 39 ];
           [ 27; 36 ];
           [ 41; 48 ];
           [ 17; 42 ];
           [ 40; 32 ];
           [ 2; 28 ];
           [ 35; 38 ];
           [ 3; 9 ];
           [ 41; 30 ];
           [ 5; 11 ];
           [ 24; 22 ];
           [ 39; 5 ];
           [ 40; 31 ];
           [ 18; 35 ];
           [ 23; 39 ];
           [ 20; 24 ];
           [ 45; 1 ];
         ]
         29 46))
