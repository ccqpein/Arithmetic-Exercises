(ns dc
  (:require [clojure.string :as str]))

(defn upper-case? [a]
  (if (= (str a) (str/upper-case (str a)))
    true
    false))

(defn detectCapitalUse [word]
  (let [flag (if (and (upper-case? (nth word 0))
                      (upper-case? (nth word 1)))
               true
               false)
        result (atom true)]
    (doseq [cha (rest word)
            :when (not= flag (upper-case? cha))]
      (reset! result false))
    result))

(detectCapitalUse "USA")
(detectCapitalUse "FlaG")
(detectCapitalUse "ggg")

