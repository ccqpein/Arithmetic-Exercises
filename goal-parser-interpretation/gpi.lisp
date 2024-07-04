(defun interpret (command)
  (loop with flag = nil
        and result = '()
        for c across command
        do (cond ((char= #\( c) (setf flag t))
                 ((char= #\) c)
                  (when flag (setf result (append result '(#\o))))
                  (setf flag nil))
                 (t (setf result (append result (list c))
                          flag nil)))
        finally (return (concatenate 'string result))))

(defun main ()
  (assert (string= "Goal" (interpret "G()(al)")))
  (assert (string= "Gooooal" (interpret "G()()()()(al)")))
  (assert (string= "alGalooG" (interpret "(al)G(al)()()G")))
  )
