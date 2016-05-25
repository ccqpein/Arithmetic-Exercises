(defun add-digits (num)
  (labels ((func (num)
             (let* ((str (write-to-string num))
                    (result
                     (loop for s across str sum
                          (digit-char-p s))))
               (if (< result 10)
                   (return-from add-digits result)
                   (func result)))))
    (func num)))
                    
