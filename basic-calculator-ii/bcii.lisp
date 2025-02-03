(defun calculate (s)
  (do* ((s (clean s))
        (x (car s) (car s))
        (y (cadr s) (cadr s))
        (bbb))
       ((null s)
        ;;(format t "bbb: ~a~%" (reverse bbb))
        (plus-and-minus (reverse bbb)))
    ;;(format t "~a~%" s)
    (if (or (string= y "*")
            (string= y "/"))
        (do* ((ss s (cddr ss))
              (x (car ss) (car ss))
              (y (cadr ss) (cadr ss))
              (bucket))
             ((or (string= y "+")
                  (string= y "-")
                  (null y))
              (push x bucket)
              (push (format nil "~a" (mul-and-div (reverse bucket))) bbb)
              (setf s (cdr ss)))
          ;;(format t "ss: ~a~%" ss)
          (push x bucket)
          (push y bucket))
        (progn (push x bbb)
               (setf s (cdr s)))
        )))

(defun clean (s)
  (let (bucket
        res)
    (loop for c across s
          do (cond ((equal c #\ )
                    (push (concatenate 'string bucket) res)
                    (setf bucket nil))
                   ((or (equal c #\/)
                        (equal c #\+)
                        (equal c #\-)
                        (equal c #\*))
                    (push (concatenate 'string bucket) res)
                    (push (string c) res)
                    (setf bucket nil))
                   (t
                    (setf bucket (append bucket (list c))))
                   ))
    (push (concatenate 'string bucket) res)
    (reverse (remove "" res :test #'string=))))

(defun plus-and-minus (s)
  (let ((init (parse-integer (car s))))
    (loop for (op v) on (cdr s) by #'cddr
          do (cond ((string= op "+")
                    (incf init (parse-integer v)))
                   (t
                    (decf init (parse-integer v)))))
    init))

(defun mul-and-div (s)
  (let ((init (parse-integer (car s))))
    (loop for (op v) on (cdr s) by #'cddr
          do (cond ((string= op "*")
                    (setf init (* init (parse-integer v))))
                   (t
                    (setf init (floor (/ init (parse-integer v)))))))
    init))

(defun main ()
  (format t "~a~%" (calculate "3+2*2"))     ;; 7
  (format t "~a~%" (calculate "2*2"))       ;; 4
  (format t "~a~%" (calculate " 3/2 "))     ;; 1
  (format t "~a~%" (calculate " 3+5 / 2 ")) ;; 5
  (format t "~a~%" (calculate " 14/3*2 "))  ;; 8
  )
