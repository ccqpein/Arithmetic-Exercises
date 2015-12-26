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

(defun isHappy (num)
  (let* ((numList (splitNumber num))
         (numListRemember)
         (isHappyy (numReturn numList)))
    ;;;;;fix
    (if (= (length numList) 1)
        (if (= isHappyy 1)
            (return-from isHappy 'true)
            (return-from isHappy 'nil))
        (isHappy isHappyy))))
        
