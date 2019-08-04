(defun dominant-index (nums)
  (let ((more-largest nil)
        (largest-v 0)
        (largest-ind 0)
        )
    (loop
       with ind = 0
       for n in nums
       do
         (cond
           ((>= n (* 2 largest-v))
            (setf largest-ind ind
                  largest-v n
                  more-largest nil))
           ((> n largest-v)
            (setf more-largest t
                  largest-v n))
           ((> (* 2 n) largest-v)
            (setf more-largest t)))
       do (incf ind))
    (if more-largest -1 largest-ind)
    ))

(dominant-index '(3 6 1 0))
(dominant-index '(1 2 3 4))
