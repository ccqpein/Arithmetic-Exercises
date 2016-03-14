(defvar *matrix* '((9 9 4) (6 6 8) (2 1 1)))

(defun find-Ele (matrix row col)
  (let ((ele))
    (if  (and (>= row 0) (>= col 0))
         (progn (setf ele  (nth col (nth row matrix)))
                (if ele
                    (return-from find-Ele (list ele (list row col) T)))
                (return-from find-Ele nil)))))

(defun do-around (func matrix row col)
  (let ((rowlist (list (1+ row) (1- row)))
        (collist (list (1+ col) (1- col)))
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

(defmacro change-index-to-string (r c)
  "return value type is r-c"
  `(intern (coerce (list (code-char (+ 48 ,r)) #\- (code-char (+ 48 ,c))) 'string)))

(defun gen-index-table (matrix)
  (let ((rowNum (length matrix))
        (colNum (length (car matrix)))
        (indexHash (make-hash-table)))
    (loop for r from 0 to (1- rowNum) do
       (loop for c from 0 to (1- colNum) do
            (progn ;(print (change-index-to-string r c))
              (setf (gethash (change-index-to-string r c) indexHash)
                (how-many-larger (do-around 'find-ele *matrix* r c))))))
    indexHash))

(defun sum-list (list)
  (let ((summation 0))
    (dolist (i list summation)
      (setf summation (+ i summation)))))

(defun longest-list (&rest llist)
  (let ((length-list '()))
    (loop for i in llist do
         (setf length-list (append length-list (length i))))
    (max 

         
  

(defun tree-search-matrix (indexTable r c)
  (let ((this-hash (gethash (change-index-to-string r c) indexTable))
        (result))
  (cond ((= 0 (car this-hash))
         (return '(1) ))
        ((= 1 (car this-hash))
         (return (setf result
                       (append '(1) (tree-search-matrix indexTable
                                                        (nth 1 (nth 1 (this-hash)))
                                                        (nth 2 (nth 1 (this-hash))))))))
        ((= 2 (car this-hash))
         (return (setf result (longest-list 
                       (append '(1) (tree-search-matrix indexTable
                                                        (nth 1 (nth 1 (this-hash)))
                                                        (nth 2 (nth 1 (this-hash)))))
                       (append '(1) (tree-search-matrix indexTable
                                                        (nth 1 (nth 2 (this-hash)))
                                                        (nth 2 (nth 2 (this-hash)))))))))
        ((= 2 (car this-hash))
         (return (setf result (longest-list 
                       (append '(1) (tree-search-matrix indexTable
                                                        (nth 1 (nth 1 (this-hash)))
                                                        (nth 2 (nth 1 (this-hash)))))
                       (append '(1) (tree-search-matrix indexTable
                                                        (nth 1 (nth 2 (this-hash)))
                                                        (nth 2 (nth 2 (this-hash)))))))
        
#|
(defun main (matrix)
  (let ((resultList '())
        (indexTable (gen-index-table matrix))
        (rowNum (length matrix))
        (colNum (length (car matrix))))
    (loop for r from 0 to (1- rowNum) do
         (loop for c from 0 to (1- colNum) do
              (
|#
