(def testStr1 "25525511135")
(def testStr2 "0000")
(def testStr3 "010010")
(def testStr4 "101023")

(defn filter-ip [ip]
  "return if ip (string) input is legal ip address"
  (cond (= ip "") false
        (> (Integer. ip) 255) false
        (and (> (count ip) 1) (= \0 (nth ip 0))) false
        :else
        true))

(defn find-next [ipstr doneip]
  "give total ip string and ip done, return list of next possible ip"
  (let [allpossible (for [i (range 1 4) ;index of next 3 digits
                          :let [len (count doneip)
                                end (if (>= (+ len i) (count ipstr)) ;if out of range
                                      (count ipstr)
                                      (+ len i))]]
                      (subs ipstr len end))] ;cut ip string
    ;(println allpossible)
    (distinct (for [ip allpossible
                    :when (filter-ip ip)]
                ip))))

(defn restore-Ip-Addresses [ipstr]
  (let [pool (atom [[]])]
    (doseq [_ (range 4)] ;4 parts of ip
      (reset! pool
              (for [onegroupip @pool
                    nextIp (find-next ipstr (reduce str onegroupip))]
                (conj onegroupip nextIp)))
      ;(println @pool)
      )
    ;filter results which length is match of ip string
    (reset! pool
            (for [ip @pool
                  :when (= (count ipstr) (reduce + (map count ip)))]
              ip))
    (println "result: ")
    (distinct @pool)))


(println (restore-Ip-Addresses testStr1))
(println (restore-Ip-Addresses testStr2))
(println (restore-Ip-Addresses testStr3))
(println (restore-Ip-Addresses testStr4))
