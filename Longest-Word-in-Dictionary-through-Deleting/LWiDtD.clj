(defn find-Longest-Word [s d]
  (let [newD (sort-by count > d) ; newD is a list
        scan-Word (fn [sIn s]
                    (if (> (count s) (count sIn))
                      nil
                      (loop [a (seq sIn)
                             b (seq s)]
                        (cond (empty? a) nil
                              (= (first a) (first b))
                              (if (empty? (rest b))
                                true
                                (recur (rest a) (rest b)))
                              :else (recur (rest a) b)))))]
    ;; main part is different from python version
    ;; because newD is sorted by alphabet in clojure, but reversed-alphabet in python version
    (loop [sIn s
           words newD]
      (cond (empty? words) ""
            (scan-Word sIn (first words)) (first words)
            :else (recur sIn (rest words))))))


(find-Longest-Word "abpcplea" ["ale" "apple" "monkey" "plea"])
(find-Longest-Word "abpcplea" ["a" "b" "c"])
(find-Longest-Word "apple" ["zxc" "vbn"])
