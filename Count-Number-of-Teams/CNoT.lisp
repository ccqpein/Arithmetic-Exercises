(defun num-teams (rating)
  (let* ((len (length rating))
         (cache (loop
                  for ind from 0 to (1- len)
                  for larger = (count-if (lambda (x) (> x (nth ind rating)))
                                          (subseq rating (1+ ind)))
                  collect (cons larger (- len 1 ind larger))))
         (result 0))
    (loop
      for ind from 0 to (- len 3)
      do (loop
           for j from (1+ ind) to (- len 2)
           if (> (nth j rating) (nth ind rating))
             do (incf result (car (nth j cache)))
           else
             do (incf result (cdr (nth j cache)))))
    result))

(defun main ()
  (assert (= 3 (num-teams '(2 5 3 4 1))))
  (assert (= 0 (num-teams '(2 1 3))))
  (assert (= 4 (num-teams '(1 2 3 4)))))
