(defn smallest-good-base [n]
  (let [num (BigInteger. n)] ;1E9 is big integer
    (loop [l (range (inc (Math/floor (/ (Math/log num) (Math/log 2))))
                    2 -1)
           k (Math/floor (Math/pow num (/ 1 (dec (first l)))))]
      ;(println l k) ;for debug
      (cond (empty? l) (str (dec num))
            (== num (/ (dec (Math/pow k (first l))) (dec k)))
            (str k)
            (= 1 (count l)) ;when l only has one element ':else' will has issue
            (str (dec num))
            :else
            (recur (rest l) (->> (first (rest l))
                                 (dec) (/ 1) (Math/pow num) (Math/floor)))))))

(smallest-good-base "13")
(smallest-good-base "4681")
(smallest-good-base "1000000000000000000")
