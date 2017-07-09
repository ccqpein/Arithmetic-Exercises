;;; Using the better solution in README

(defn numSquares [num]
  (if (< num 2) num)
  (let [lst (atom #{})
        toCheck (atom #{})]
    ()))
