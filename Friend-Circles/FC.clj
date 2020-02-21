(def test1 [[1 1 0]
             [1 1 0]
             [0 0 1]]) ; 2

(def test2 [[1 1 0]
             [1 1 1]
             [0 1 1]]) ; 1

(def test3 '[[1 0 0 1]
             [0 1 1 0]
             [0 1 1 1]
             [1 0 1 1]]) ; 1

(def test4 [[1 1 0]
             [1 1 0]
             [0 0 1]]) ; 2

(defn doIndex [ind m pool]
  (let [result (atom pool)]
    (doseq [singleset (for [i (range 0 (count m))
                       :when 
                       (and (= 1 (nth (nth m ind) i))
                            (not (contains? pool i)))]
                        (doIndex i m (conj pool i)))]
      (swap! result clojure.set/union singleset))
    @result))


(defn friend-circle [m]
  (let [stuNum (count m)
        pool (atom #{})
        result (atom 0)]
    (doseq [ind (range 0 stuNum)]
      (if (not (contains? @pool ind))
        (do (swap! result inc)
            (reset! pool (doIndex ind m @pool)))))
    result))
