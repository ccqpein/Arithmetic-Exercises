(defun longest-common-subsequence (text1 text2)
  (let ((text1 (concatenate 'list text1))
        (text2 (concatenate 'list text2))
        (dp (make-list (length text1) :initial-element 0)))
    (loop for i from 0 below (length text2)
          for am = nil
          and sc = 1
          do (loop for j from 0 below (length text1)
                   if (<= sc (nth j dp))
                     do (setf am nil
                              sc (1+ sc))
                   else if (and (not am) (equal (nth i text2) (nth j text1)))
                          do (setf (nth j dp) sc
                                   am t)))
    (apply #'max dp)))

(defun main ()
  (assert (= 3 (longest-common-subsequence "abcde" "ace")))
  (assert (= 3 (longest-common-subsequence "abc" "abc")))
  (assert (= 0 (longest-common-subsequence "abc" "def")))
  )

