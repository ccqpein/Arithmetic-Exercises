(defn next-greater-elements [nums]
  (let [len (count nums)
        inds (take len (range))
        res (atom (vec (to-array (repeat len -1) )))]
    (loop [stack []
           indexs inds]
      (cond (empty? indexs) @res
            (and stack (< (nth nums (last stack))
                          (nth nums (first indexs))))
            ))))
