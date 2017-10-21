(defn find-shortest-subarray [nums]
  (loop [store-all-ele {}
         stack-of-degree [0 []]
         nums nums
         ind 0]
    (cond (empty? nums)
          (apply min
                 (map #(- (last (last (get store-all-ele %)))
                          (first (last (get store-all-ele %))))
                      ))
          :else
          (recur (try
                   (if (> (count (last (get store-all-ele (first nums)))) 1)
                     (update store-all-ele (first nums)
                             (conj (last (get store-all-ele (first nums))) ind))))))))

(defn update-store)
