(require '[clojure.string :as str])

(defn complexNumberMultiply [a b]
  (let [aL (str/split a #"\+" 2)
        bL (str/split b #"\+" 2)
        ai (Integer. (first aL))
        bi (Integer. (first bL))
        ac (Integer. (subs (last aL) 0 (- (count (last aL)) 1)))
        bc (Integer. (subs (last bL) 0 (- (count (last bL)) 1)))
        ins (- (* ai bi) (* ac bc))
        com (+ (* ai bc) (* bi ac))]
    (str ins "+" com "i")
    ))
















