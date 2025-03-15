(defun maximum-count (nums)
  (loop for ind from 0 below (length nums)
        for i in nums
        if (< i 0)
          count 1 into neg
        else
          if (> i 0)
            return (max neg (- (length nums) ind))
        finally (return neg)))

(defun main ()
  (assert (eq 3 (maximum-count '(-2 -1 -1 1 2 3))))
  (assert (eq 3 (maximum-count '(-3 -2 -1 0 0 1 2))))
  (assert (eq 4 (maximum-count '(5 20 66 1314)))))
