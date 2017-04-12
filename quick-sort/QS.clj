(defn quick-sort [l] ;; l = array
  (let [key (first l)]
    (if (= 0 (count l))
      []
     (vec (concat
           (conj (quick-sort (filter #(if (< % key) %) (vec (rest l)))) key)
           (quick-sort (filter #(if (>= % key) %) (vec (rest l)))))))))

(quick-sort [2 3 4 5 2 1 4 5 3])
