(defun convert-to-title-old (n)
  (do ((temp-list (cons (mod n 26) '()) (cons (mod rest 26) temp-list))
       (rest (floor n 26) (floor rest 26)))
      ((= rest 0) temp-list)
    ))

(defun main ())
