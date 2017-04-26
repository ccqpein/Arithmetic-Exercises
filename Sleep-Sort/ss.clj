(def store (atom []))

(defn sleep-num [n]
  (Thread/sleep (* n 1000))
  (swap! store conj n))

(defn main [nl]
  (pmap sleep-num nl)
  @store)

(main [1 5 6 4 3 2 7])
