(defun three-consecutive (predict ll)
  (if (< (length ll) 3)
      nil
      (if (and (funcall predict (nth 0 ll))
               (funcall predict (nth 1 ll))
               (funcall predict (nth 2 ll)))
          t
          (three-consecutive predict (cdr ll)))))
