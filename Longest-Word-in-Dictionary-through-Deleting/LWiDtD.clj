(defn find-Longest-Word [s d]
  (let [newD (sort-by count > d) ; newD is a list
        tempList (atom [])
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
    (loop [sIn s
           words newD]
      (cond (empty? words) ""
            (scan-Word sIn (first words)) (first words)
            :else (recur sIn (rest words))))))


(find-Longest-Word "abpcplea" ["ale" "apple" "monkey" "plea"])
(find-Longest-Word "abpcplea" ["a" "b" "c"])
(find-Longest-Word "apple" ["zxc" "vbn"])
