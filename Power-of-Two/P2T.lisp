(defun is_power_of_two (n)
  (if (zerop n)
      nil
      (let ((a 1))
        (loop
          repeat 32
          if (eq (logand a n) n)
            do (return-from is_power_of_two t)
          do (setf a (ash a 1))))))

(defun main ()
  (assert (is_power_of_two 16))
  (assert (is_power_of_two 1))
  (assert (not (is_power_of_two 3)))
  (assert (is_power_of_two 4))
  (assert (not (is_power_of_two 5)))
  (assert (not (is_power_of_two 0)))
  (assert (not (is_power_of_two -2147483648)))
  )
