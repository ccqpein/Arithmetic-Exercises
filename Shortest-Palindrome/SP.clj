(defn palindrome-p [l-char]
  (let [len (count l-char)
        head (if (odd? len)
               (butlast (subvec l-char 0
                                (int (Math/ceil
                                      (/ len 2)))))
               (subvec l-char 0
                       (int (Math/ceil
                             (/ len 2)))))
        tail (subvec l-char
                     (int (Math/ceil
                           (/ len 2))))]
    (if (not head)
      (= tail [])
      (= head (reverse tail)))))

(require '[clojure.string :as str])

(defn shortest-palindrome [str]
  (if (= "" str) ""
      (let [l-char (vec str)
            re-l-char (vec (reverse l-char))]
        (loop [i re-l-char
               ind 0]
          (if (palindrome-p i)
            (str/join (concat (subvec re-l-char 0 ind) l-char))
            (recur (vec (rest i)) (inc ind)))
          ))))

(= (shortest-palindrome "aacecaaa") "aaacecaaa")
(= (shortest-palindrome "abcd") "dcbabcd")
(= (shortest-palindrome "") "")
(= (shortest-palindrome "aba") "aba")
(= (shortest-palindrome "abbacd") "dcabbacd")
(= (shortest-palindrome "abb") "bbabb")
(= (shortest-palindrome "aaaaa") "aaaaa")
(= (shortest-palindrome "aabba") "abbaabba")
(= (shortest-palindrome "a") "a")
(= (shortest-palindrome "babbbabbaba") "ababbabbbabbaba")
(= (shortest-palindrome "aaaabbaa") "aabbaaaabbaa")
