(defun valid-mountain-array (arr)
  (let ((a 0)
        (b (1- (length arr))))
    (loop for (x y) on arr by #'cdr
          while y
          if (> y x)
            do (incf a)
          else
            return nil)
    (loop for (x y) on (reverse arr) by #'cdr
          while y
          if (> y x)
            do (decf b)
          else
            return nil)
    (format t "~a, ~a~%" a b)
    (and (= a b)
         (/= a 0)
         (/= a (1- (length arr))))))

(defun main ()
  (assert (not (valid-mountain-array '(2 1))))
  (assert (not (valid-mountain-array '(3 5 5))))
  (assert (valid-mountain-array '(0 3 2 1)))
  (assert (not (valid-mountain-array '(2 0 2)))))
