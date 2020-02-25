(defun print-hashmap (m)
  (loop
     for k being the hash-keys
     using (hash-value v) of m
       do (format t "~a: ~a~%" k v)))

(defun find-circle-num (m)
  (let ((row-num (length m))
        (col-num (length (car m)))
        (friends-map (make-hash-table :test 'equal))
        (already-set (make-hash-table))
        (result 0))
    
    (loop
       for r from 0 to (1- row-num)
       do (loop
             for c from 0 to (1- col-num)
             do (if (eq 1 (nth c (nth r m)))
                    (setf (gethash r friends-map) (cons c (gethash r friends-map))))))

    ;;(print-hashmap friends-map)

    (loop
       for k being the hash-keys 
       using (hash-value v) of friends-map
       do (if (not (gethash k already-set))
              (progn (incf result)
                     (insert-set already-set friends-map v))))
    
     result
    ))

(defun insert-set (set m vl)
  (dolist (v vl) (setf (gethash v set) t))
  (loop
     for v in vl
     do (insert-set set m (remove-if (lambda (x) (gethash x set)) (gethash v m)))))
