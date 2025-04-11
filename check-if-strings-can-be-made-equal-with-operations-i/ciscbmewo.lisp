;;(ql:quickload "alexandria")

(defun can-be-equal (s1 s2)
  (let ((set0 (make-hash-table :test 'equal))
        (set1 (make-hash-table :test 'equal))
        (set2 (make-hash-table :test 'equal))
        (set3 (make-hash-table :test 'equal)))
    (setf (gethash (elt s1 0) set0) t
          (gethash (elt s1 2) set0) t
          (gethash (elt s2 0) set1) t
          (gethash (elt s2 2) set1) t

          (gethash (elt s1 1) set2) t
          (gethash (elt s1 3) set2) t
          (gethash (elt s2 1) set3) t
          (gethash (elt s2 3) set3) t)

    (and (= 0 (length (hash-set-difference set0 set1)))
         (= 0 (length (hash-set-difference set2 set3))))
    ))

(defun hash-set-difference (set other)
  (loop for k being the hash-keys of set
        if (not (gethash k other))
          collect k))
