(defn soe [n]
  (let [nL (atom (range 1 (+ 1 n)))
        endNum (Math/ceil (Math/sqrt 5))]
    (doseq [i (range 2 (+ 1 endNum))]
      (swap! nL (fn [l]
                  (remove
                   (fn [x]
                     (and (not= x i ) (= (mod x i) 0))) l))
             ))
    (rest @nL)))
