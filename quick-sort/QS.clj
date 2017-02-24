(defn quick-sort [l] ;; l = array
  (let [key (first l)]
    (if (= 0 (count l))
      []
     (into [] (concat
               (conj (quick-sort (filter #(if (< % key) %) (into [] (rest l)))) key)
               (quick-sort (filter #(if (>= % key) %) (into [] (rest l)))))))))
