(defun minimum-difference (nums k)
  (if (< (length nums) 2)
      0
      (let ((nums (sort nums #'<)))
        (loop
          for i from 0 to (- (length nums) k)
          for a = (- (nth (+ i k -1) nums)
                     (nth i nums))
          minimize a))))
