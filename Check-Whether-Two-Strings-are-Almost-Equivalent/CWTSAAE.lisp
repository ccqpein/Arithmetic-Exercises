(ql:quickload "alexandria")

(defun check-whether-two-strings-are-almost-equivalent (word1 word2)
  (let ((table1 (make-hash-table))
        (table2 (make-hash-table)))
    (dolist (c (concatenate 'list word1))
      (incf (gethash c table1 0)))
    (dolist (c (concatenate 'list word2))
      (incf (gethash c table2 0)))
    ;;(format t "~a" (alexandria:hash-table-plist table1))
    ;;(format t "~a" (alexandria:hash-table-plist table2))
    (and (loop
           for k being the hash-key of table1
             using (hash-value v)
           if (> (abs (- v (gethash k table2 0))) 3)
             do (return-from check-whether-two-strings-are-almost-equivalent nil)
           finally (return t))
         (loop
           for k being the hash-key of table2
             using (hash-value v)
           if (> (abs (- v (gethash k table1 0))) 3)
             do (return-from check-whether-two-strings-are-almost-equivalent nil)
           finally (return t)))
    ))

(defun main ()
  (assert (not (check-whether-two-strings-are-almost-equivalent "aaaa" "bccb")))
  (assert (check-whether-two-strings-are-almost-equivalent "abcdeef" "abaaacc"))
  (assert (check-whether-two-strings-are-almost-equivalent "cccddabba" "babababab"))
  (assert (not (check-whether-two-strings-are-almost-equivalent "zzzyyy" "aaaaaa"))))
