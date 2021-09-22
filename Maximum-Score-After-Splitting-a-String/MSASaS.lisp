(defun max-score (s)
  (let* ((sl (concatenate 'list s
                          ))
         (count (if (char= (nth 0 sl) #\0) 1 0))
         (total-one (if (char= (car (last sl)) #\1) 1 0))
         (max-count count))
    (loop
      for ind from 1 below (1- (length sl))
      for c = (nth ind sl)
      do (ccase c
           (#\0
            (incf count)
            (if (> count max-count) (setf max-count count)))
           (#\1 (decf count) (incf total-one)))
      finally (return (+ total-one max-count))
     )))

(defun main ()
  (assert (= 5 (max-score "011101")))
  (assert (= 5 (max-score "00111")))
  (assert (= 3 (max-score "1111")))
  (assert (= 1 (max-score "00")))
  )
