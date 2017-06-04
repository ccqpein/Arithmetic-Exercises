(defn check-Subarray-Sum [nums k]
  (let [len (count nums)]
    (cond (<= len 1) false
          (= 0 (apply + nums)) true
          (= 0 k) false
          (and (> k 0) (>= len (* 2 k))) true
          :else
          (let [storeAllRemainder (loop [head [0]
                                         ind 0]
                                    (if (= len ind)
                                      head
                                      (recur (conj head
                                                   (mod (+ (last head)
                                                           (nth nums ind))
                                                        k))
                                             (inc ind))))
                tempDic (atom {})
                result (atom false)]
            (doseq [ind (take (count storeAllRemainder) (range))
                    :let [thisElm (get @tempDic (nth storeAllRemainder ind))]]
              ;(print thisElm)
              (if thisElm
                (if (and (> ind (inc thisElm))
                         (not @result))
                  (reset! result true))
                (swap! tempDic conj {(nth storeAllRemainder ind) ind})
                ))
            @result))))


(check-Subarray-Sum [23 2 4 6 7] 6)
(check-Subarray-Sum [23 2 6 4 7] -6) 
(check-Subarray-Sum [0 0] -1) 
(check-Subarray-Sum [1 1] 2) 
(check-Subarray-Sum [1 2 3] 4)  
(check-Subarray-Sum [1 2 3 2 10 2] 14) 
