(defun move-zeroes (numList)
  (let* ((numList2 '())
	 (times 0))
    (dolist (x numList)
      (if (/= x 0)
	  (setq numList2 (append numList2 (list x)))
	  (incf times 1)))
    (setq numList2 (append numList2 (make-list times :initial-element '0)))
    (format t "~S" numList2)))


(defun move-zeroes2 (numList)
  (let ((reL '()))
    (dolist (x (reverse numList))
      (if (/= x 0)
          (setf reL (append (list x) reL))
          (setf reL (append reL (list x)))))
    reL))

