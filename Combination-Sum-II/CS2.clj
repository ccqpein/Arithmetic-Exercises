(defn combination-sum2 [cands target]
  (let [cands (sort cands)
        list-of-sets (doall (for [_ (range (inc target))]
                              (atom #{})))]
    
    (swap! (nth list-of-sets 0) (fn [x] (conj x [])))

    (doseq [cand cands
            t (range target (dec cand) -1)
            prev @(nth list-of-sets (- t cand))]

      (swap! (nth list-of-sets t)
             conj
             (conj prev cand)))
    
    @(last list-of-sets)
    ))
