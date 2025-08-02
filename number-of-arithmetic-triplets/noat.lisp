(ql:quickload "alexandria")

(defun arithmetic-triplets (nums diff)
  (let ((vs (make-hash-table :test 'equal)))
    (loop for ind from 0 below (length nums)
          for v in nums
          do (push ind (gethash v vs)))
    (pprint (alexandria:hash-table-plist vs))

    (loop with res = 0
          for ind from 0 below (length nums)
          for n in nums
          if (and (gethash (+ diff n) vs) (gethash (+ diff diff n) vs))
            do (incf res (* (length (gethash (+ diff n) vs))
                            (length (gethash (+ diff diff n) vs))))
          finally (return res))))

(arithmetic-triplets '(0 1 4 6 7 10) 3)
(arithmetic-triplets '(4 5 6 7 8 9) 2)

