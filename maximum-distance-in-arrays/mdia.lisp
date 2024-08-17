(defun max-distance (l)
  (let (left-edge right-edge (result 0))
    (loop for i from 0 below (length l)
          do (push (list (first (nth i l)) i) left-edge)
          do (push (list (car (last (nth i l))) i) right-edge))

    (setf left-edge (sort left-edge #'<)
          right-edge (sort right-edge #'>))
    
    (if (/= (second (first right-edge))
            (second (first left-edge)))
        (setf result (max result (abs (- (first (first right-edge)) (first (first left-edge)))))))
    
    (if (/= (second (second right-edge))
            (second (first left-edge)))
        (setf result (max result (abs (- (first (second right-edge)) (first (first left-edge)))))))

    (if (/= (second (first right-edge))
            (second (second left-edge)))
        (setf result (max result (abs (- (first (first right-edge)) (first (second left-edge)))))))

    (if (/= (second (second right-edge))
            (second (second left-edge)))
        (setf result (max result (abs (- (first (second right-edge)) (first (second left-edge)))))))

    result
    ))

(defun main ()
  (assert (= 4 (max-distance '((1 2 3) (4 5) (1 2 3)))))
  (assert (= 0 (max-distance '((1) (1)))))
  )
