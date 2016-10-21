(defun hash+1 (key hashT)
  "plus 1 if key-vaule existed, else new let key-value be 1"
  (if (gethash key hashT)
      (incf (gethash key hashT))
      (setf (gethash key hashT) 1)))


(defun make-ftd-hash (s)
  (let ((sh (make-hash-table)))
    (loop for c across s
       do (hash+1 c sh))
    sh))

(defun find-the-difference (s p)
  (let ((sT (make-ftd-hash s))
        (pT (make-ftd-hash p)))
    (loop for k being the hash-key of pT
       when (/= (gethash k pT)
                (if (gethash k sT)
                    (gethash k sT)
                    0))
       do (print k))))
