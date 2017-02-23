(defn mergee [l1 l2]
  (cond(empty? l1)
       l2
       (<= (first l1) (first l2))
       (conj (mergee (drop 1 l1) l2) (first l1))
       :else
       (conj (mergee (drop 1 l2) l1) (first l2))))


(defn mergeSort [l]
  (let [len (count l)
        [temp1 temp2] (split-at (quot len 2) l)]
    (if (>= len 3)
      (mergee (mergeSort temp1) (mergeSort temp2))
      (mergee temp1 temp2))
    ))

(def a '(1 2 3 2 4 3 2 4 2 1 3 4 3 2 1 2 4 5 6 7 7 5 4 5 4 2 2))

(mergeSort a)
