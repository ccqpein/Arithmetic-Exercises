(require '[clojure.string :as str])

(defn reverse-string [str]
  (let [str-list (str/split str #" ")]
    (str/join " " (reverse str-list))
    ))

(reverse-string " aa bb")
