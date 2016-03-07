(defun find-Ele (matrix row col)
  (let ((ele (nth col (nth row matrix))))
  (if ele
      (return-from find-Ele (list ele (list row col) T))
      (return-from find-Ele nil))))

(defun do-around (func matrix row col)
  (let ((rowlist (list (+ 1 row) (- 1 row)))
        (collist (list (+ 1 col) (- 1 col)))
        (result))
    (setf result
          (list
           (funcall func matrix row col)
           (funcall func matrix (nth 0 rowlist) col)
           (funcall func matrix (nth 1 rowlist) col)
           (funcall func matrix row (nth 0 collist))
           (funcall func matrix row (nth 1 collist))))
    result))

(defun how-many-larger (resultFromDoAround)
  (let ((thisNum (caar resultFromDoAround))
        (lastList (cdr resultFromDoAround))
        (largerNum 0))
    (dolist (i lastList largerNum)
      (if (> (car i) thisNum)
          (incf largerNum)))))    
