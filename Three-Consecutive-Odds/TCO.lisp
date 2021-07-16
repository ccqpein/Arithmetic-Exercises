(defun three-consecutive (predict ll)
  (if (< (length ll) 3)
      nil
      (if (and (funcall predict (nth 0 ll))
               (funcall predict (nth 1 ll))
               (funcall predict (nth 2 ll)))
          t
          (three-consecutive predict (cdr ll)))))

;; (defun num-consecutive (num)
;;   (labels ((inner (predict ll)
;;              (if (< (length ll) num)
;;                  nil
;;                  (if (reduce (lambda (x y) (and x y))
;;                              (loop
;;                                for i from 0 downto num
;;                                collect (funcall predict (nth i ll))))
;;                      t
;;                      (inner predict (cdr ll))))))
;;     inner))
