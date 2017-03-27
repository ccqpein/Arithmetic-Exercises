(defn checkPerfectNumber [num]
  (cond (<= num 1) false
        :else
        (let [upper (-> num (Math/sqrt) (Math/floor) (+ 1))
              sumNum (atom 0)]
          (doseq [i (range 1 upper)
                :when (= (mod num i) 0)
                :let [ii (/ num i)]]
            (swap! sumNum + i ii))
          (if (= num (- @sumNum num))
            true
            false))))
