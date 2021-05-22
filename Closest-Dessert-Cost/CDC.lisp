(defun closest-cost (base toppings target)
  (last-step
   (sort (loop
           for b in base
           append (cost-val b toppings target)
           )
         #'<)
   target))

(defun cost-val (base toppings target)
  (cond ((>= base target)
         (list base))
        ((= 0 (length toppings))
         (list base))
        (t
         (append nil
                 (cost-val base (cdr toppings) target)
                 (cost-val (+ base (car toppings)) (cdr toppings) target)
                 (cost-val (+ base (* 2 (car toppings))) (cdr toppings) target)))))

(defun last-step (l target)
  (let ((diff most-positive-fixnum)
        (v 0))
    (loop
      for d in l
      if (or (< (abs (- d target)) diff)
             (= v d))
        do (setf diff (abs (- d target))
                 v d)
      else
        do (return-from last-step v))))
