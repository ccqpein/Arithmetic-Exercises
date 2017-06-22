(require '(clojure [string :as str] [set :as set]))


(defn find-min-difference [timePoint]
  (let [minDict (apply hash-map (flatten
                                 (for [time (map #(str/split % #":") timePoint)]
                                   (map read-string time))))]
    minDict))

(find-min-difference ["23:59" "00:00"])
