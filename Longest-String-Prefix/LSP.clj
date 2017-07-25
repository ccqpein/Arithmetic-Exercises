(def alphabet {	"a" 1 "b" 2 "c" 3 "d" 4	"e" 5 "f" 6 "g" 7 "h" 8 "i" 9  "j" 10 "k" 11 "l" 12 "m" 13 "n" 14 "o" 15 "p" 16 "q" 17 "r" 18 "s" 19 "t" 20 "u" 21 "v" 22 "w" 23 "x" 24 "y" 25 "z" 26})

(defn longest-prefix [s]
  (loop [strseq (seq s)
         stack 0
         templong 0
         longest 0]
    ;println (first strseq) stack templong longest)
    (cond (= strseq '())
          (if (and (= 0 longest) (not (= 0 templong))) templong longest)
          (>= (get alphabet (str (first strseq))) stack)
          (recur (rest strseq) (get alphabet (str (first strseq))) (inc templong) longest)
          :else
          (recur (rest strseq)
                 (get alphabet (str (first strseq)))
                 1
                 (if (> templong longest) templong longest)))))

(println (longest-prefix "knotty"))
(println (longest-prefix "apple"))
(println (longest-prefix "excel"))







