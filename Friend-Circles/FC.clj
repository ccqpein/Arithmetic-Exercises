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


(defn find-circle [m]
  (let [totalNum (count m)
        circle (atom 0)
        pool (atom (set (range 0 totalNum)))]
    (while (-> @pool empty? not)
      (let [this (first @pool)]
        ((fn [ind]
           (loop [indIn ind]
             (doseq [i (range 0 indIn)]
               (if (and (-> m (nth indIn) (nth i) (= 1))
                        (contains? @pool i))
                 (do (swap! pool #(set (remove %)) #{i})
                     (recur (first @pool))))))) this))
      (swap! pool inc))))
