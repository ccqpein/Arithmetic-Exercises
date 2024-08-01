(defun can-alice-win (nums)
  (/= (loop for n in nums when (< n 10) sum n)
      (loop for n in nums when (>= n 10) sum n))
  )
