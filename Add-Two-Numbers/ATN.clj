(defn add-two-number [x y]
  (loop [a x
         b y
         re []]
    (print x y re)
    (cond (and (empty? a) (empty? b))
          re
          (empty? a)
          (into [] (concat re b))
          (empty? b)
          (into [] (concat re b))
          :else
          (recur (into [] (rest a))
                 (into [] (rest b))
                 (conj re (+ (first a) (first b)))))))

(defn check-ten [l]
  (let [f (atom 0)
        ll (doall (for [a l]
                    (let [inner-a (+ a @f)]
                      (if (>= inner-a 10)
                        (do (reset! f (int (Math/floor (/ inner-a 10))))
                            (mod inner-a 10))
                        inner-a
                        ))))]
    (if (not= 0 @f)
      (conj (into [] ll) @f)
      (into [] ll))))

(add-two-number [2 6 8] [8 9 3 1])
(check-ten [10 15 11 1])

(check-ten (add-two-number [9 9 9 9] [9 9 9 9]))
