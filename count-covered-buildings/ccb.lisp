(defun count-covered-buildings (n buildings)
  (let ((x (make-hash-table))
        (y (make-hash-table)))
    (loop for b in buildings
          for this-x = (nth 0 b)
          and this-y = (nth 1 b)
          do (let ((x-en (gethash this-x x (list n 0)))
                   (y-en (gethash this-y y (list n 0))))
               (setf (gethash this-x x)
                     (list (min (nth 0 x-en) (nth 1 b))
                           (max (nth 1 x-en) (nth 1 b)))

                     (gethash this-y y)
                     (list (min (nth 0 y-en) (nth 0 b))
                           (max (nth 1 y-en) (nth 0 b))))))

    (- (length buildings)
       (loop for b in buildings
             for yy = (gethash (nth 0 b) x)
             and xx = (gethash (nth 1 b) y)
             unless (and (/= (nth 1 b) (nth 0 yy))
                         (/= (nth 1 b) (nth 1 yy))
                         (/= (nth 0 b) (nth 0 xx))
                         (/= (nth 0 b) (nth 1 xx)))
               sum 1))))

(defun main ()
  (assert (= 0 (count-covered-buildings 3 '((1 2) (2 1) (3 1) (1 1) (3 3) (3 2))))))
