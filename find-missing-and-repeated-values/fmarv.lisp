(defun find-missing-and-repeated-values (grid)
  (let ((cache (make-list (* (length grid) (length grid)) :initial-element 0))
        (dup 0))
    (loop for l in grid
          do (loop for v in l
                   if (= 1 (nth (1- v) cache))
                     do (setf dup v)
                   else
                     do (incf (nth (1- v) cache))))
    (list dup (loop for ind from 0 below (length cache)
                    if (zerop (nth ind cache))
                      return (1+ ind)))))

(defun main ()
  (assert (equal '(2 4) (find-missing-and-repeated-values '((1 3) (2 2)))))
  (assert (equal '(9 5) (find-missing-and-repeated-values '((9 1 7) (8 9 2) (3 4 6))))))
