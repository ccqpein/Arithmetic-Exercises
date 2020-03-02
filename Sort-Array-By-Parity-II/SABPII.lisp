(defun sort-array-by-parity-ii (al)
  (let ((even (sort (remove-if-not #'evenp al) #'<))
        (odd (sort (remove-if-not #'oddp al) #'<)))
    (format t "~a, ~a~%" even odd)
    (do ((even-rest even (cdr even-rest))
         (odd-rest odd (cdr odd-rest))
         (result '()))
        ((and (not even-rest) (not odd-rest)) (reverse result))
      (print result)
      (if even-rest (push (car even-rest) result))
      (if odd-rest (push (car odd-rest) result)))))

(defun main ()
  (print (sort-array-by-parity-ii '(4 2 5 7))))

