(defun array-pair-sum (nums)
  (let ((sorted-nums (sort nums #'<)))
    (loop
      for v in sorted-nums by #'cddr
      sum v)
    ))
