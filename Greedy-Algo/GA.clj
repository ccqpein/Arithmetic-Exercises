(def data [[1 2] [2 4] [3 4] [3 6] [3 5] [4 7] [4 5] [7 8] [6 9]])

(defn GA [data]
  (let [sortData (sort-by second data)]
    (reverse (reduce (fn [a b]
                       (if (>= (first b) (second (first a)))
                         (conj a b)
                         a))
                     (list (first sortData))
                     (rest sortData)))))
