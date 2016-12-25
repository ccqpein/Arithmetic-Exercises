(def *test1 '())
(def *test2 '(2 1 2 1 0 1 2))
(def *test3 '(1))
(def *test4 '(7 1 5 3 6 4))
(def *test5 '(2 4 1))

(defn max-profit [prices]
  (def ^:dynamic maxv)
  (def ^:dynamic minv)
  (def ^:dynamic result)
  (def restl (try (pop prices)
                  (catch Exception e "0")))
  (try
    (binding [maxv 0
              minv (first prices)
              result 0]
      (doseq [i restl]
        (cond
          (< i minv) (set! minv i)
          (> (- i minv) result) (do (set! maxv i)
                                    (set! result (- maxv minv)))))
      result)
    (catch Exception e "0")))

