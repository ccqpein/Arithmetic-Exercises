(require '[clojure.string :as str])

(defn replace-words [dict sentence]
  (let [sentlist (str/split sentence #" ")]
    (loop [dict dict
           sentlist sentlist]
      (cond (empty? dict) sentlist
            :else 
            (recur (rest dict)
                   (map #(if (= (first dict)
                                (try (subs % 0 (count (first dict)))
                                     (catch StringIndexOutOfBoundsException e "")))
                           (first dict)
                           %) sentlist))))))

(replace-words '("cat" "bat" "rat") "the cattle was rattled by the battery")
