(defvar *matrix* '((9 9 4) (6 6 8) (2 1 1)))

(defun find-Ele (matrix row col)
  (let ((ele))
    (if  (and (>= row 0) (>= col 0))
         (progn (setf ele  (nth col (nth row matrix)))
                (if ele
                    (return-from find-Ele (list ele (list row col) T)))
                (return-from find-Ele nil)))))

(defun do-around (func matrix row col)
  (let ((rowlist (list (+ 1 row) (- 1 row)))
        (collist (list (+ 1 col) (- 1 col)))
        (result))
    (setf result (remove 'nil
          (list
           (funcall func matrix row col)
           (funcall func matrix (nth 0 rowlist) col)
           (funcall func matrix (nth 1 rowlist) col)
           (funcall func matrix row (nth 0 collist))
           (funcall func matrix row (nth 1 collist)))))
    result))

(defun how-many-larger (resultFromDoAround)
  "return lagerest number and the index"
  (let ((thisNum (caar resultFromDoAround))
        (lastList (cdr resultFromDoAround))
        (largerNum 0)
        (largerIndex '()))
    (dolist (i lastList (list largerNum largerIndex))
      (if (> (car i) thisNum)
          (progn (incf largerNum)
                (setf largerIndex (append largerIndex (list (cadr i))))
          )))))

(defun main (matrix)
  (let ((rowNum (length matrix))
        (colNum (length (car matrix)))
        (indexHash (make-hash-table)))
    (loop for r from 0 to (1- rowNum) do
       (loop for c from 0 to (1- colNum) do
            (progn (print (list r c))
              (setf (gethash (list r c) indexHash)
                (how-many-larger (do-around 'find-ele *matrix* r c)))
         (print (gethash (list r c) indexHash)))))
    (print indexHash)))
