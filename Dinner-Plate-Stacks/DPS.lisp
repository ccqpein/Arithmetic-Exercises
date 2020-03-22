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

(defun dp-pop (dp)
  (declare (type dinner-plates dp))
  (the integer
       (loop
          for i from (dp-pop-point dp) downto 0
          if (/= 0 (length (nth i (dp-stacks dp))))
          do (setf (dp-pop-point dp) i)
          and do (return-from dp-pop (dp-pop-at-stack dp i))
          finally (return -1))))

(defun dp-pop-at-stack (dp i)
  (declare (type dinner-plates dp))
  (cond ((>= i (dp-stacks-num dp))
         (return-from dp-pop-at-stack -1))
        ((not (car (nth i (dp-stacks dp))))
         -1)
        (t
         (if (< i (dp-push-point dp))
             (setf (dp-push-point dp) i))
         (pop (nth i (dp-stacks dp)))
         )))

(defun test (commands numbers)
  (let (dp)
    (labels ((handle (c n)
               (cond ((string= c "DinnerPlates") (setf dp (make-dinner-plates :cap n)))
                     ((string= c "push") (dp-push dp n))
                     ((string= c "popAtStack") (dp-pop-at-stack dp n))
                     ((string= c "pop") (dp-pop dp))
                     )))
     (loop
        for c in commands
        for n from 0 
        collect (handle c (nth n numbers)) into result
        finally (return (values result num))))))

;; (test '("DinnerPlates" "push" "push" "push" "push" "push" "push" "push" "popAtStack" "popAtStack" "popAtStack" "popAtStack" "popAtStack" "push" "push" "pop" "pop" "pop" "pop" "pop") '(2 1 2 3 4 5 6 7 2 2 1 1 0 8 9))
;;(#S(DINNER-PLATES
;;     :CAP 2
;;     :STACKS (NIL NIL NIL NIL)
;;     :STACKS-NUM 4
;;     :PUSH-POINT 0
;;     :POP-POINT 0)
;;  NIL NIL NIL NIL NIL NIL NIL 6 5 4 3 2 NIL NIL 7 9 8 1 -1)

;; (test '("DinnerPlates" "push" "push" "popAtStack" "pop" "push" "push" "pop" "pop") '(1 1 2 1 nil 1 2))
;;(#S(DINNER-PLATES
;;    :CAP 1
;;    :STACKS (NIL NIL)
;;    :STACKS-NUM 2
;;    :PUSH-POINT 0
;;    :POP-POINT 0)
;; NIL NIL 2 1 NIL NIL 2 1)
