(def *test1 '())
(def *test2 '(2 1 2 1 0 1 2))
(def *test3 '(1))
(def *test4 '(7 1 5 3 6 4))
(def *test5 '(2 4 1))

(defn max-profit [prices]
  (def ^:dynamic maxv 0)
  (def ^:dynamic minv (first prices))
  (def ^:dynamic result 0)
  (def restl (pop prices))
  (doseq [i restl]
    (println i)
    (cond
      (< i minv) (set! minv i)
      (> (- i minv) result) (do (set! maxv i)
                                (set! result (- maxv minv))
                                (print "hi" result)))
    (print "{" maxv minv "}"))
  (set! result (if (< result 0) 0 result))
  result)

