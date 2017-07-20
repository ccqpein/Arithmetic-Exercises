(defn is-ugly [num]
  (cond (contains? [0 -1] num) true
        (contains? [1 2 3 5] num) true
        (= 0 (mod num 2)) (recur (/ num 2))
        (= 0 (mod num 3)) (recur (/ num 3))
        (= 0 (mod num 5)) (recur (/ num 5))
        :else false))


(is-ugly 6)
(is-ugly 8)
(is-ugly 14)
