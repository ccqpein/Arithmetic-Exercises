(defun SN (nL)
  (let ((hs (make-hash-table)))
    (loop for i in nL
       do (cond ((not (eql nil (gethash i hs)))
                 (setf (gethash i hs) (1+ (gethash i hs))))
                ('t
                 (setf (gethash i hs) 1))))
    (loop for k being the hash-keys of hs
       using (hash-value v)
       do (if (/= v 2)
              (return-from SN k)))))
