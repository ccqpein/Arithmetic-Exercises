(defvar *m '((1 2 1)
             (0 2 3)
             (9 8 7)))

(defmacro with-gensyms ((&rest names) &body body)
  `(let ,(loop for n in names collect `(,n (gensym)))
               ,@body))

(defmacro aappend (l &rest eles)
  "l must be symbol not expression. For example, (aappend a 2 3 4) is fine, (aappend '(1 2) 2 3) and (aappend (list 2 3) 2 2) will issue error"
  (with-gensyms (elel)
    `(let ((,elel (list ,@eles)))
       (loop for i in ,elel do
            (setf ,l (append ,l (list i))))
       ,l)))

(defun setZeroes (m)
  (let* ((rowNum (length m))
         (colNum (length (nth 0 m)))
         (zeroList (make-list colNum :initial-element 0)))
    (do* ((ind 0 (1+ ind))
          (colZeroIndex '())
          (thisRow (elt m ind) (elt m ind)))
         ((= ind rowNum) (print m))
      (let ((zeroK nil))
        (loop for i from 0 to (1- rowNum) do
             (if (= 0 (elt thisRow i))
                 (progn (setf zeroK t)
                        (if (not (find i colZeroIndex))
                            (aappend colZeroIndex i)))))
        (if zeroK (setf (elt m ind) zeroList))
        (loop for i from 0 to ind do
             (loop for ii in colZeroIndex do
                  (setf (elt (elt m i) ii) 0)))))))
