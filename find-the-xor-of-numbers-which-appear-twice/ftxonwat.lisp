(defun duplicate-numbers-xor (nums)
  (loop with set = (make-hash-table)
        and aa = 0
        for n in nums
        do (setf aa (logxor aa n)
                 (gethash n set) t)
        finally (return (logxor aa (loop with bb = 0
                                         for k being the hash-key of set
                                         do (setf bb (logxor bb k))
                                         finally (return bb))))))

(defun main ()
  (assert (= 1 (duplicate-numbers-xor '(1 2 1 3))))
  (assert (= 0 (duplicate-numbers-xor '(1 2 3))))
  (assert (= 3 (duplicate-numbers-xor '(1 2 2 1)))))
