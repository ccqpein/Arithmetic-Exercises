(defun splitNumber (num)
  (let* ((numStr (write-to-string num))
         (numList (mapcar #'digit-char-p (coerce numStr 'list))))
    numList))

(defun numReturn (numList)
  (let ((sumNum
    (apply #'+ 
           (mapcar #'(lambda (x)
                       (* x x))
                   numList))))
    sumNum))

(defvar *numListRemember* nil)

(defun isHappy (num)
  (let* ((numList (splitNumber num))
         (isHappyy (numReturn numList)))
    (if (member isHappyy *numListRemember*)
        (return-from isHappy 'nil)
        (if (= isHappyy 1)
            (return-from isHappy 'true)))
    (push isHappyy *numListRemember*)
    (isHappy isHappyy)))
