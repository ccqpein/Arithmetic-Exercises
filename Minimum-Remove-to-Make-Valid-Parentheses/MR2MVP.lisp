(defun min-remove-to-make-valid (s)
  (let ((head '())
        (tail '())
        (pre-s (concatenate 'list s))
        all-notch)
    (loop
      for c in pre-s
      for ind from 0
      do (cond ((char= #\( c)
                (push ind head))
               ((char= #\) c)
                (if (/= 0 (length head))
                    (pop head)
                    (push ind tail)))))
    (setf all-notch (sort (append head tail) #'<))
    ;;(format t "all notch ~a~%" all-notch)
    (concatenate 'string
                 (loop
                   for c in pre-s
                   for ind from 0
                   if (eql ind (car all-notch))
                     do (setf all-notch (cdr all-notch))
                   else
                     collect c))))

(defun main ()
  (format t "~a~%" (min-remove-to-make-valid "lee(t(c)o)de)"))
  (format t "~a~%" (min-remove-to-make-valid "a)b(c)d"))
  (format t "~a~%" (min-remove-to-make-valid "))(("))
  (format t "~a~%" (min-remove-to-make-valid "(a(b(c)d)")))
