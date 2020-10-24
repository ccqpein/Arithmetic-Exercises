(defun mct-from-leaf-values (arr)
  (let ((record (make-hash-table :test 'equal)))
    (second (mct-inner-help arr 0 (length arr) record))
    ))

(defun mct-inner-help (arr start-ind end-ind record)
  (if (gethash (list start-ind end-ind) record)
      (gethash (list start-ind end-ind) record)
      ;;else
      (cond ((zerop (- end-ind start-ind))
             '(0 0))

            ((= 1 (- end-ind start-ind))
             (setf (gethash (list start-ind end-ind) record)
                   (list (nth start-ind arr) 0))
             (list (nth start-ind arr) 0))

            ((= 2 (- end-ind start-ind))
             (let ((x (max (nth start-ind arr)
                           (nth (1+ start-ind) arr)))
                   (y (* (nth start-ind arr) (nth (1+ start-ind) arr))))
               (setf (gethash (list start-ind end-ind) record)
                     (list x y))
               (list x y)))

            (t
             (let ((cache (minimum (loop
                                     for i from 1 below (- end-ind start-ind)
                                     for cache-l = (let* ((cache0 (mct-inner-help arr
                                                                                  start-ind
                                                                                  (+ i start-ind)
                                                                                  record))
                                                          (cache1 (mct-inner-help arr
                                                                                  (+ i start-ind)
                                                                                  end-ind
                                                                                  record))
                                                          (x (max (car cache0) (car cache1)))
                                                          (y (+ (* (car cache0) (car cache1)) (cadr cache0) (cadr cache1))))
                                                     (list x y))
                                     collect cache-l)
                                   #'< #'cadr)))
               (setf (gethash (list start-ind end-ind) record) (list (car cache) (cadr cache)))
               (list (car cache) (cadr cache)))))))

;;; https://stackoverflow.com/questions/30273802/how-would-i-get-the-min-max-of-a-list-using-a-key
(defun minimum (list predicate key)
  (when list
    (let* ((m0 (first list))
           (m1 (funcall key m0)))
      (mapc (lambda (e0 &aux (e1 (funcall key e0)))
              (when (funcall predicate e1 m1)
                (psetf m0 e0 m1 e1)))
            list)
      m0)))

