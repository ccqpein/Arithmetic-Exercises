(defn is-anagram [s t]
  (cond (= "" s) false
        (= (seq s) (->> t (seq) (reverse))) true
        :else false))

(is-anagram "abc" "cba")
(is-anagram "abc" "cba ")
