(ql:quickload "split-sequence")

(defun complex-number-multiply (a b)
  (let (a-real
        b-real
        a-complex
        b-complex)
    (multiple-value-setq (a-real a-complex) (parse-it a))
    (multiple-value-setq (b-real b-complex) (parse-it b))

    (let ((real-result (+ (* a-real b-real )
                          (* -1 a-complex b-complex)))
          (complex-result (+ (* a-real b-complex)
                             (* b-real a-complex))))
      (format nil "~a+~ai" real-result complex-result))))

(defun parse-it (a)
  (multiple-value-bind (result num)
      (split-sequence:split-sequence #\+ a)
    (values (parse-integer (car result))
            (parse-integer (car (split-sequence:split-sequence #\i (cadr result)))))))
