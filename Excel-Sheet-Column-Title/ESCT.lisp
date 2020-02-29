(defun convert-to-title-old (n)
  (do ((temp-list (cons (mod n 26) '()) (cons (mod rest 26) temp-list))
       (rest (floor n 26) (floor rest 26)))
      ((= rest 0)
       (mapcar #'convert-char (remove 0 (parse-lst temp-list))))
    ))

(defun parse-lst (ll)
  (let ((flag 0)
        (result '()))
    (dolist (e (reverse ll))
      (cond ((= 0 e)
             (setf result (cons (+ flag 26) result)
                   flag -1))
            (t
             (setf result (cons (+ e flag) result)
                   flag 0
                   ))))
    result))

(defun convert-char (n)
  (code-char (+ n 64)))

(defun main ()
  (format t "~a~%" (convert-to-title-old 28))
  (format t "~a~%" (convert-to-title-old 1))
  (format t "~a~%" (convert-to-title-old 701)))
