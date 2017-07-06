(def testStr1 "25525511135")
(def testStr1 "0000")
(def testStr1 "010010")
(def testStr1 "101023")

(defn filter-ip [ip]
  (cond (> (Integer. ip) 255) false
        (and (> (count ip) 1) (= \0 (nth ip 0))) false
        :else
        true)))

(defn find-next [ipstr doneip]
  "give total ip string and ip done, return next possible ip"
  (let [allpossible (for [i (range 1 4)
                          :let [len (count doneip)]]
                      (subs ipstr len (+ len i)))]
    (for [ip allpossible
          :when (filter-ip ip)]
      ip)))


(defn restore-Ip-Addresses [ipstr]
  (let [ippool (atom [])]))







