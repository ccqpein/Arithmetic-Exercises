(defn heapify [func nl ind] ; return new nl and child index
  (let [parent (nth nl ind) ; nl cannot be empty
        leftind (inc (* 2 ind))
        left (if (>= leftind (count nl)) nil (nth nl leftind))
        rightind (+ 2 (* 2 ind))
        right (if (>= rightind (count nl)) nil (nth nl rightind))]
    (cond
      (nil? left) [nl nil]
      (and (func parent left)
           (if (nil? right) true (func parent right)))
      [nl nil]
      (and (func left parent)
           (if (nil? right) true (func left right)))
      [(assoc nl leftind parent ind left) leftind]
      :else
      [(assoc nl rightind parent ind right) rightind]
      )))

;(print (heapify >= [4 1 3 2 16 9 10 14 8 7] 4))

(defn parent-ind [ind]
  (if (= -1 ind) -1
      (-> ind dec (/ 2) int)))

(defn build-heap [func arr]
  (loop [arr arr
         stack [(parent-ind (dec (count arr)))]]
    ;(println arr stack)
    (cond
      (= -1 (first stack)) arr
      :else
      (let [[newarr newind] (heapify func arr (last stack))]
        (recur newarr
               (cond newind (conj stack newind)
                     :else
                     [(dec (first stack))]))))))

(println (build-heap >= [4 1 3 2 16 9 10 14 8 7]))

(defn heap-sort [func arr]
  (loop [result []
         arr arr]
    (cond (= [nil] arr) result
          :else
          (let [[this & remaining] (build-heap func arr)]
            (print remaining)
            (recur (conj result this)
                   (into [] (conj (butlast remaining) (last remaining))))))))

(println (heap-sort >= [4 1 3 2 16 9 10 14 8 7]))
(println (heap-sort <= [4 1 3 2 16 9 10 14 8 7]))
