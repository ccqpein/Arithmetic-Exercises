(defn kill-process [pid ppid kill]
  (let [tempdic (atom {})]
    (doseq [i (range (count ppid))]
      (if (get @tempdic (nth ppid i))
        (swap! tempdic update (nth ppid i) conj (nth pid i))
        (swap! tempdic conj {(nth ppid i) [(nth pid i)]})))
    (loop [stack (list kill)
           rest '()]
      ;(println stack)
      (cond (empty? stack) rest
            :else
            (recur (flatten (for [this stack
                                  :let [value (get @tempdic this)]
                                  :when value]
                              value))
                   (concat rest stack))))
    ))

(kill-process [1 3 10 5] [3 0 5 3] 5)
(kill-process [1 3 10 5] [3 0 5 3] 3)
