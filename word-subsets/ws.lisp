(defun word-subsets (words1 words2)
  (let ((a (make-list 26 :initial-element 0)))
    (loop for w in words2
          do (setf a (merge-array a (make-word-array w))))
    (loop for w in words1
          if (compare-array (make-word-array w) a)
            collect w))
  )

(defun make-word-array (word)
  (loop with re = (make-list 26 :initial-element 0)
        for c across word
        do (incf (nth (- (char-code c) (char-code #\a)) re))
        finally (return re)))

(defun merge-array (a1 a2)
  (loop for x in a1
        for y in a2
        collect (max x y)))

(defun compare-array (a b)
  (loop for aa in a
        for bb in b
        if (< aa bb)
          return nil
        finally (return t)))

(pprint (word-subsets '("amazon" "apple" "facebook" "google" "leetcode")
                      '("e" "o")))
(pprint (word-subsets '("amazon" "apple" "facebook" "google" "leetcode")
                      '("lc" "eo")))

(pprint (word-subsets '("acaac" "cccbb" "aacbb" "caacc" "bcbbb")
                      '("c" "cc" "b")))
