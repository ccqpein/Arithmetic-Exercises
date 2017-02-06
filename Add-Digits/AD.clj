(defn add-digits [str]
  (let [n (read-string str)]
    (loop [numL n]
      (let [sumN (reduce + (spilit-num numL))]
        (if (< sumN 10)
          sumN
          (recur sumN))))))

(defn spilit-num [num]
  "split num digit by digit, like 123 => 1 2 3"
  (loop [a (quot num 10)
         b (mod num 10)
         result (list b)]
    (if (< a 10)
      (conj result a)
      (recur (quot a 10) (mod a 10) (conj result (mod a 10))))))
