;;; borrow the function from my toolbox
;;; https://github.com/ccqpein/CLisp-toolbox-ccQ/blob/master/general-tool.lisp#L38

(defun update-hash (key hash-table fun &rest args)
  "update hash value conveniently"
  (setf (gethash key hash-table)
        (apply fun (gethash key hash-table) args)
        ))

(defun set-hash (key hash-table newVal)
  (setf (gethash key hash-table) newVal))

(defun kill-process (pid ppid kill)
  (let ((tempHash (make-hash-table))
        (result '())
        (stack (list kill)))
    (loop for ind from 0 to (length ppid)
       for thisP = (nth ind ppid)
       for thisChindP = (nth ind pid)
       do (if (gethash thisP tempHash)
              (setf (gethash thisP tempHash)
                    (append (gethash thisP tempHash)
                            (list thisChindP)))
              (set-hash thisP tempHash (list thisChindP))
              ))
    (labels ((get-from-stack (stack)
               (let ((tempRe (loop for id in stack
                                collect (gethash id tempHash))))
                 (loop with result = '()
                    for lis in tempRe
                    do (setf result (append result lis))
                    finally (return result))))
             (add-result (stack result)
               (if (eql nil stack)
                   result
                   (add-result (get-from-stack stack)
                               (append result stack)))))
      ;(print (get-from-stack stack))
      (add-result stack result))
    ))
