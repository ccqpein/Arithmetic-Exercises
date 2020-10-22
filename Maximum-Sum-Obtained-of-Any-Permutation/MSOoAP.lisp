(defun max-sum-range-query (nums requests)
  (let ((count-l (make-list (1+ (length nums)) :initial-element 0)))
    (loop
      for (i j) in requests
      do (incf (nth i count-l))
      do (decf (nth (1+ j) count-l)))

    (loop
      for i from 1 to (length nums)
      do (setf (nth i count-l)
               (+ (nth i count-l) (nth (1- i) count-l))))

    (loop
      with result = 0
      for num in (sort nums #'<)
      for i in (sort (butlast count-l) #'<)
      do (incf result (mod (* num i) 1000000007))
      do (setf result (mod result 1000000007))
      finally (return result)
      )
    ))

(defun main ()
  (assert (= 47 (max-sum-range-query '(1 2 3 4 5 10)
                                     '((0 2) (1 3) (1 1)))))
  (assert (= 11 (max-sum-range-query '(1 2 3 4 5 6)
                                     '((0 1)))))
  (assert (= 19 (max-sum-range-query '(1 2 3 4 5)
                                     '((1 3) (0 1))))))
