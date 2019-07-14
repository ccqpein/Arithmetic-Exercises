(quicklisp:quickload 'split-sequence)

(defun corp-flight-bookings (bookings n)
  (let ((result (loop for i from 0 to 20000 collect 0)))
    (loop
       for (s e v) in bookings
       do (loop
             for ind from s to e
             for original = (nth ind result)
             do (setf (nth ind result) (+ v original))))
    (cadr (split-sequence:split-sequence 0 result :start 0 :end (+ 1 n)))))
