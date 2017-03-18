(def testList [1, [3, [2]], [[[5]]]])
(def testList2 [1, [3, [2], [6, 7]], [[4, [5]]]])

(defn NL [lis]
  (let [innerList (atom lis)
        reLis (atom [])]
    (while (not (empty? @innerList))
      (let [firstE (first @innerList)]
        (cond (= firstE [])
              (reset! innerList (into [] (rest @innerList)))
              (instance? java.lang.Long firstE)
              (do (swap! reLis conj firstE)
                  (reset! innerList (into [] (rest @innerList))))
              :else
              (reset! innerList (into [] (concat firstE (rest @innerList))))
              )))
    reLis))
