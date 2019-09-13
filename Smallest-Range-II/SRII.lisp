(defun smallest-range-ii (a k)
  (if (< (length a) 2) (return-from smallest-range-ii 0))
  (let* ((a-sorted (sort a #'<))
         (mi (nth 0 a-sorted))
         (ma (nth (1- (length a)) a-sorted))
         (dis (- ma mi)))
    (do* ((rest a-sorted (cdr rest))
          (aa (car rest) (car rest))
          (bb (cadr rest) (cadr rest))
          )
         ((not (cdr rest)))
      (setf dis (min dis
                     (- (max
                         (- ma k)
                         (+ aa k))
                        (min
                         (+ mi k)
                         (- bb k)))
                     ))
      )
    dis))

(smallest-range-ii '(1) 0)
(smallest-range-ii '(0 10) 2)
(smallest-range-ii '(1 3 6) 3)

