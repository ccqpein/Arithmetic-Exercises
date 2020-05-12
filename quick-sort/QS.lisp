(defun quick-sort (l)
  (let ((key (car l)))
    (if (eql nil l)
        '()
        (append (quick-sort
                 (loop for i in (cdr l) when (< i key)
                    collect i))
                (list key)
                (quick-sort
                 (loop for i in (cdr l) when (>= i key)
                    collect i))))))


(defun quick-sort-2 (l start end)
  (if (>= start end) (return-from quick-sort-2))
  (let ((copy-start start)
        (copy-end end))
    
    (loop while (/= start end)
          do (if (> (nth start l) (nth end l))
                 (let ((temp (nth end l)))
                   (setf (nth end l) (nth start l)
                         end (1- end)
                         (nth start l) (nth end l)
                         (nth end l) temp))
                 (setf start (1+ start))))

    (quick-sort-2 l copy-start (1- start))
    (quick-sort-2 l (1+ end) copy-end)
    ))
