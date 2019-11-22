(defun jf (k)
  (if (inner 1 k)
      (return-from jf 1))

  (do ((n 1 (1+ n)))
      ((inner (* n (1+ k)) k) (* n (1+ k)))))

(defun inner (m k)
  (do ((flex-m m)
       (left (* 2 k) (1- left))
       (sub 0))
      ((= k left) t)
    ;;(format t "~a ~a~%" flex-m left)
    (cond
      ((= 0 (mod flex-m left)) (setf sub 0))
      ((<= (mod flex-m left) k) (return-from inner nil))
      (t (setf sub (- left (mod flex-m left)))))
    (setf flex-m (- m sub))))
