(ns sucs)

(def test1 '(1 2 3))
(def test2 '(1))
(def test3 '(2 6 4 8 10 9 15))
(def test4 '(2 6 4 8 10 15 9))
(def test5 '())
(def test6 '(3 2 1))

(defn find-unsorted-subarray [nums]
  (let [newNums (sort < nums)
        result (atom 0)]
    (doseq [[n o]
            (map list newNums nums)
            :when (not= n o)]
      (swap! result inc))
    @result))

(find-unsorted-subarray test3)
