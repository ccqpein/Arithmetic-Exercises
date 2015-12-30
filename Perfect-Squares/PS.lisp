(defun make-squares-list (n)
  (let ((squareList))
    (setf squareList (mapcar #'(lambda (x)
                                 (* x x))
                             (loop for i from 0 to n collect i)))
    (setf squareList (loop for item in squareList
                        when (<= item n)
                          collect item))
    squareList))

(defun sum-list (tempList squareList)
  (let ((tempList2 (list)))
    (loop for item in tempList do
       (loop for ii in squareList do
            (setf tempList2 (append tempList2 (list (+ item ii))))))
    (return-from sum-list tempList2)))

(defun gene-table (n squareList)
  (do* ((timee n (decf n))
        (tempList squareList (sum-list tempList squareList)))
       ((<= timee 1) (return-from gene-table tempList))))

(defun ps (n)
  (let ((basicSquaresTable (make-squares-list n)))
    (loop for i from 2 to 4 do
         (if (member n (gene-table i basicSquaresTable))
             (return-from ps i)))))


