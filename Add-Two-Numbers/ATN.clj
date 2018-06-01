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
