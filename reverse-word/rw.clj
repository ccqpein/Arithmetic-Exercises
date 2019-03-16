(defn reverse-word [s]
  (clojure.string/join
   ""
   (loop [[char & rest-s] s
          cache '()
          result []]
     (cond
       (nil? char) (vec (concat result cache))
       (= char \ ) (recur rest-s '() (conj (vec (concat result cache)) \ ))
       :else (recur rest-s (cons char cache) result))
     ))
  )

(reverse-word  " abc ggga ")
