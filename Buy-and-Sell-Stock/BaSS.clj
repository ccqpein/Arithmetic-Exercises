(def *test1 '())
(def *test2 '(2 1 2 1 0 1 2))
(def *test3 '(1))
(def *test4 '(7 1 5 3 6 4))
(def *test5 '(2 4 1))

(defn max-profit [prices]
  (let [maxv 0
        result 0
        minv (first prices)]
    (doseq [i prices]
      (print i)
      (cond
        (< (- i minv) 0) (def minv i)
        (> (- i minv) result) (do (def maxv i)
                                  (def result (- maxv minv))
                                  (print result)))
      (print maxv minv))
    (def result (if (< result 0) 0 result))
    result))




















