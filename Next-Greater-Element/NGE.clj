(defn next-greater-elements [nums]
  (let [len (count nums)
        inds (take len (range))
        res (atom (vec (to-array (repeat len -1) )))]
    (loop [stack '()
           indexs (concat inds inds)]
      (cond (empty? indexs)
            @res
            (and (not (empty? stack)) (< (nth nums (last stack))
                            (nth nums (first indexs))))
            (do (swap! res assoc (last stack) (nth nums (first indexs)))
                (recur (butlast stack) indexs))
            :else
            (recur (concat stack (list (first indexs))) (rest indexs))
            ))))

(next-greater-elements [1 2 1])
(next-greater-elements [5 3 4 6 7])
