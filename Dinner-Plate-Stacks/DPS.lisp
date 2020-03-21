(defstruct (dinner-plates (:conc-name dp-))
  (cap 0 :type integer)
  (stacks (list '()))
  (stacks-num 1 :type integer)
  (push-point 0 :type integer)
  (pop-point 0 :type integer))

(defun dp-push (dp val)
  (declare (type dinner-plates dp))
  
  (if (< (length (nth (dp-push-point dp)
                      (dp-stacks dp)))
         (dp-cap dp))
      (progn (push val (nth (dp-push-point dp) (dp-stacks dp)))
             (return-from dp-push nil)))

  (loop
     if (eq (dp-push-point dp) (1- (dp-stacks-num dp)))
     do (progn (setf (dp-stacks dp) (append (dp-stacks dp) (list ())))
               (incf (dp-stacks-num dp))
               (incf (dp-pop-point dp)))
       
     do (incf (dp-push-point dp))
       
     if (> (dp-push-point dp) (dp-pop-point dp))
     do (setf (dp-pop-point dp) (dp-push-point dp))

     ;;(format t "~a~%" (dp-push-point dp))
     if (< (length (nth (dp-push-point dp) (dp-stacks dp))) (dp-cap dp))
     do (push val (nth (dp-push-point dp) (dp-stacks dp)))
     and do (return-from dp-push nil))
  )

(defun pd-pop ())
(defun pd-pop-at-stack ())
