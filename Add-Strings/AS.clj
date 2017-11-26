(def chars [\1 \2 \3 \4 \5 \6 \7 \8 \9 \0])
(def ints [1 2 3 4 5 6 7 8 9 0])
(def dict (merge (zipmap chars ints)
                 (zipmap ints chars)))

(defn add-strings [& nums]
  (print nums)
  (num-to-string (apply + (map string-to-num nums))))

(defn string-to-num [^String str]
  (loop [strl (seq str)
         len (count strl)
         ind 0
         sum 0]
    ;(println strl sum)
    (cond (empty? strl) sum
          :eles
          (recur (rest strl)
                 len
                 (inc ind)
                 (+ sum (* (get dict (first strl))
                           (loop [count (- len ind 1)
                                  this 1]
                             (cond (= count 0) this
                                   :else
                                   (recur (- count 1) (* this 10))))))))))

(defn num-to-string [^Integer int]
  (str int))

