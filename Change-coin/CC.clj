;;; Because the code I showed in class writed by Haskell, which is harder to understand than C-like language.
;;; Furthermore Haskell IO part is weird, so this clojure code is Plan B

;;; You can run this code in (https://www.tutorialspoint.com/execute_clojure_online.php)
;;; Copy code in and click execute

(def coinList [1 5 12 25])
(def resultList (atom {})) ; define a independent map(dict) to store all results

;;; core function below
(defn changeCoin
  ;; The part below define defult arguments, so this function use three arguments:
  ;; 1. total val(val) 2. coin value list(coinL) which define upper 3. result map
  [val & {:keys [coinL resultL]
          :or {coinL coinList
               resultL resultList}}]
  ;; The part below use 'cond' to judge value input
  (cond (some #(= val %) coinL) 1 ; if value equal one of coins' value, return 1
        (contains? @resultL val) (get @resultL val) ; if value already in results map, return directly
        :else (do
                (swap! resultL ; update result map (cont.)
                       conj [val ; with adding new record, key is 'val', the value is (cont.)
                             ; smallest value in value list, for loop return a list
                             (apply min (for [c coinL 
                                              :when (< c val)]
                                          (+ 1 (changeCoin (- val c) coinL resultL))))]) ; where the recursive happen
                (get @resultL val)))) ; At last, return the number of coins of this value


;;; main function for running code directly
(comment "(defn main []
   (printf \"The number of 16 change coins is %s%n\" (changeCoin 16))
   (printf \"The number of 64 change coins is %s%n\" (changeCoin 64))
   (printf \"Whole map of change coin is %s%n\" (into (sorted-map) @resultList)))")

;(main)
                                        ; run main function
