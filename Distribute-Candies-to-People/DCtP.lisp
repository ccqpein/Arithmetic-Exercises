(defun distribute-candies (candies num-people)
  (let* ((result (make-list num-people :initial-element 0))
         (temp (loop for ind from 0 to (1- num-people) collect ind))
         (cycle-ind (setf (cdr (last temp)) temp)))
    (loop
       with count = 1
       for d in cycle-ind
       do (if (<= count candies)
              (setf (nth d result) (+ count (nth d result)))
              (progn (setf (nth d result) (+ candies (nth d result))) (return)))
       do (setf candies (- candies count)
                count (1+ count))
         )
    result))

(defun main ())
