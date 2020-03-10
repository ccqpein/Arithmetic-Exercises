(ql:quickload "split-sequence")

(defun find-min-difference (str-l)
  (let* ((time-pair (mapcar (lambda (x) (split-sequence:split-sequence #\: x))
                            str-l))
         (pure-time (mapcar (lambda (x) (+ (* 60
                                              (parse-integer (car x)))
                                           (parse-integer (cadr x))))
                            time-pair))
         (sorted-num (sort pure-time #'<)))
    (do* ((smallest (+ (car sorted-num)
                       (- (* 60 24)
                          (car (last sorted-num)))))
          (head (car sorted-num) (car tail))
          (tail (cdr sorted-num) (cdr tail)))
         ((not tail) smallest)
      (if (< (- (car tail) head) smallest)
          (setf smallest (- (car tail) head)))
      )))

(defun main ()
  (find-min-difference '("22:27" "18:42" "09:57" "09:24" "09:26")) ;; => 2
  )
