(defn find-LHS [nums]
  (let [dic (into (sorted-map)
                  (loop [nums nums
                         dic {}]
                    (cond (empty? nums) dic
                          (get dic (first nums))
                          (recur (rest nums) (update dic (first nums) inc))
                          :else
                          (recur (rest nums) (conj dic {(first nums) 1})))))
        keysSeq (seq (keys dic))]
    (loop [result 0
           inds (rest (range (count keysSeq)))]
      (cond (empty? inds) result
            (and (= 1 (- (->> inds (first) (nth keysSeq))
                         (->> inds (first) (dec) (nth keysSeq))))
                 (< result (+ (->> inds (first) (nth keysSeq) (get dic))
                              (->> inds (first) (dec) (nth keysSeq) (get dic)))))
            (recur (+ (->> inds (first) (nth keysSeq) (get dic))
                      (->> inds (first) (dec) (nth keysSeq) (get dic))) (rest inds))
            :else
            (recur result (rest inds)))))
  )

(find-LHS [1 3 2 2 5 2 3 7])

