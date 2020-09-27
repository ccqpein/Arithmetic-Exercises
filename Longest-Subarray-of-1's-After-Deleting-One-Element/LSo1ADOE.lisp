(defun longest-subarray (nums)
  (loop
    with result = 0 and cache = 0 and count = 0
    for num in nums
    do (cond ((= 1 num)
              (incf count))
             (t
              (cond ((= 0 count)
                     (setf cache 0))
                    (t (if (> (+ count cache) result)
                           (setf result (+ count cache)))
                       (setf cache count
                             count 0))
                    )))
    finally (progn (if (> (+ count cache) result)
                       (setf result (+ count cache)))
                   (return result))
    ))

(defun main ()
  (assert (= 3 (longest-subarray '(1 1 0 1))))
  (assert (= 5 (longest-subarray '(0 1 1 1 0 1 1 0 1))))
  (assert (= 4 (longest-subarray '(1 1 0 0 1 1 1 0 1))))
  (assert (= 0 (longest-subarray '(0 0 0 0))))
  (assert (= 19 (longest-subarray '(0 0 1 0 1 0 1 1 1 1 0 1 1 1 1 1 1 1 1 0 1 1 0 0 1 1 1 1 1 0 1 1 1 1 1 1 1 1 1 1 1 1 1 1 0 0 0 1))))
  )
