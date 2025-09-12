(defun min-operations (nums)
  (if (every (lambda (x) (= x (nth 0 nums))) nums) 0 1))
