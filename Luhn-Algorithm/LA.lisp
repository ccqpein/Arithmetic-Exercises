(defun is-valid (input)
  (declare (simple-string input))
  (labels ((check-nine (n) (if (> n 9) (- n 9) n)))
    (let ((input (reverse (remove-if (lambda (x) (char= #\  x))
                                     (coerce input 'list)))))
      
      (if (<= (length input) 1) (return-from is-valid nil))
      
      (do ((flag t (not flag))
           (result 0)
           (input input (cdr input)))
          ((not input) (zerop (mod result 10)))

        (let ((v (digit-char-p (car input))))
          (if (not v) (return-from is-valid nil))
          (if flag
              (incf result v)
              (incf result (check-nine (* 2 v))))
          )))))
