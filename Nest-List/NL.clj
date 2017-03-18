(def testList [1, [3, [2]], [[[5]]]])
(def testList2 [1, [3, [2], [6, 7]], [[4, [5]]]])

(defn NL [reLis lis]
  (cond (empty? lis) reLis
        (= (first lis) []) 
        (instance? clojure.lang.PersistentVector (first lis))
        )
)
