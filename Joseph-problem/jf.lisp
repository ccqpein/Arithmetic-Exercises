(declaim (optimize (speed 3) (safety 0) (debug 0)))

(defun jf (k)
  (declare (type fixnum k))
  (if (inner 1 k)
      (return-from jf 1))

  (do* ((n 1 (1+ n))
        (m (* n (1+ k)) (* n (1+ k))))
       ((inner m k) m)
    (declare (type fixnum n m))
    ))

(defun inner (m k)
  (declare (type fixnum m k))
  (do ((flex-m m)
       (left (* 2 k) (- left 1))
       (sub 0))
      ((= k left) t)
    (declare (type fixnum left sub))
    ;;(format t "~a ~a~%" flex-m left)
    (let ((remaind (the fixnum (mod flex-m left))))
      (cond
        ((= 0 remaind) (setf sub 0))
        ((<= remaind k) (return-from inner nil))
        (t (setf sub (- left remaind))))
      (setf flex-m (- m sub)))))
