;;; Using the better solution in README
;;; A lot side effect operation LOL

(defn numSquares [num]
  (let [lst (atom #{})
        toCheck (atom #{num})
        i (atom 0)]
    (while (< (* @i @i) num)
      (do (swap! i inc)
          (swap! lst conj (* @i @i))))
    ;(println @lst)
    (loop [Check @toCheck
           cnt 1]
      ;(println Check)      
      (cond (or (contains? Check 0) (empty? Check)) 0
            (->> (clojure.set/intersection Check @lst) empty? not) cnt
            :else
            (recur (set (for [x Check y @lst :when (< y x)]
                          (- x y)))
                   (inc cnt))))))

(numSquares 12) ;3 => 4 + 4 + 4
(numSquares 13) ;2 => 4 + 9
