(defvar *m (make-array '(3 3) :initial-contents
                                '((1 2 1)
                                  (0 2 3)
                                  (9 8 7))))

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
  (let* ((rowNum (array-dimension m 0))
         (colNum (array-dimension m 1))
         (colZeroIndex '()))
    (loop for i from 0 to (1- rowNum) do
         (progn
           (let ((ifZero nil))
             (loop for ii from 0 to (1- colNum) when (= 0 (aref m i ii)) do
                                        ;(if (= 0 ele)
                  (progn (setf ifZero t) (aappend colZeroIndex ii)
                         (print colZeroIndex)))
                                        ;)
             (if (not ifZero)
                 (progn (loop for rei from 0 to (1- i) do
                             (loop for ii in colZeroIndex do
                                  (setf (aref m rei ii) 0)))
                        (loop for ii from 0 to (1- colNum) do
                             (setf (aref m i ii) 0)))
                 (loop for ii from 0 to (1- colNum) do
                      (setf (aref m i ii) 0)))
           )))))
                    
