(defun frequency-sort (nums)
  (let ((num-f (make-hash-table :test 'equal))
        (f-num (make-hash-table :test 'equal)))
    (loop for n in nums
          do (incf (gethash n num-f 0)))

    (loop for n being each hash-key of num-f
            using (hash-value f)
          do (setf (gethash f f-num)
                   (if (gethash f f-num)
                       (cons n (gethash f f-num))
                       (list n))))

    (loop for f from 1 to 100
          for nn = (gethash f f-num)
          if nn
            append (loop for n in (sort nn #'>)
                         append (make-list f :initial-element n))
              into result
          finally (return result))))

(format t "~a~%" (frequency-sort '(1 1 2 2 2 3)))
(format t "~a~%" (frequency-sort '(2 3 1 3 2)))
(format t "~a~%" (frequency-sort '(-1 1 -6 4 5 -6 1 4 1)))
