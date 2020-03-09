(ql:quickload "split-sequence")

(defun find-min-difference (str-l)
  (let ((time-pair (mapcar (lambda (x) (split-sequence:split-sequence #\: x))
                           str-l))
        ())
    ))

(defun main ())
