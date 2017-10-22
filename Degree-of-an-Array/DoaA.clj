(defn get-degree-nums [dict]
  (let [largest (atom 0)
        num-list (atom [])]
    (doseq [key-value dict]
      (cond (> (val key-value) @largest)
            (do (reset! largest (val key-value))
                (reset! num-list [(key key-value)]))
            (= (val key-value) @largest)
            (swap! num-list conj (key key-value))))
    @num-list))


(defn find-shortest-subarray [nums]
  (let [degree-num-dict (frequencies nums)
        degree-num (get-degree-nums degree-num-dict)
        store-index (atom (zipmap degree-num (repeat [])))]
    (loop [ind 0
           nums nums]
      (cond (empty? nums)
            (apply min
                   (map #(- (last (last %)) (first (last %)) -1)
                                          (into [] @store-index)))
            :else
            (do (if (some #(= (first nums) %) degree-num)
                  (cond (> (count (get @store-index (first nums))) 1)
                        (swap! store-index update-in [(first nums)]
                               #(conj (into [] (butlast %)) ind))
                        (< (count (get @store-index (first nums))) 1)
                        (swap! store-index assoc (first nums) [ind])
                        :else
                        (swap! store-index update-in [(first nums)]
                               #(conj % ind))))
                ;(println store-index)
                (recur (inc ind) (rest nums)))
            ))))

(find-shortest-subarray [1 2 2 3 1])
(find-shortest-subarray [1,2,2,3,1,4,2])

