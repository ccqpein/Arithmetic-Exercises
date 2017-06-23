;;; same design of python version

(require '(clojure [string :as str] [set :as set]))

(defn parse-int [s]
  (Integer/parseInt (re-find #"\A-?\d+" s)))

(defn find-min-difference [timePoint]
  (let [initDict (apply hash-map (flatten (for [time (range 24)] (list time #{}))))
        updateDict (loop [dict initDict
                          times timePoint]
                     (cond (empty? times)
                           (conj dict {24 (get dict 0)})
                           :else
                           (let [[hour mins] (str/split (first times) #":")]
                             (recur (conj dict {(parse-int hour)
                                                (set/union (get dict (parse-int hour))
                                                           #{(parse-int mins)})})
                                    (rest times)))))]
    ;; cannot return immediatly when add same element in Set
    (loop [thisHour 0
           small 1339]
      (cond (= thisHour 24) small
            :else
            (let [nextHourFirstMin (first (sort (get updateDict (inc thisHour))))
                  minL (if nextHourFirstMin
                         (conj (into [] (sort (get updateDict thisHour)))
                               (+ 60 nextHourFirstMin))
                         (into [] (sort (get updateDict thisHour))))]
              ;; minL is vector including this hour and the first element in next hour
              (recur (inc thisHour)
                     (if (> (count minL) 1)
                       (apply min (for [ind (range 1 (count minL))]
                                    (- (nth minL ind) (nth minL (dec ind)))))
                       small)))))))

(find-min-difference ["23:59" "00:00"])
(find-min-difference ["22:27", "18:42", "09:57", "09:24", "09:26"])

