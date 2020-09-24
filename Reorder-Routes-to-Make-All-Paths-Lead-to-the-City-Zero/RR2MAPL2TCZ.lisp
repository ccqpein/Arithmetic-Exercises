(defun min-reorder (n connections)
  (let ((set (make-hash-table :test 'equal))
        (graph (make-hash-table)))
    
    (dolist (node connections)
      (setf (gethash (list (car node) (cadr node)) set) t)
      (let ((a (gethash (car node) graph))
            (b (gethash (cadr node) graph)))
        (setf (gethash (car node) graph) (append a (list (cadr node)))
              (gethash (cadr node) graph) (append b (list (car node))))))

    (labels ((dfs (child parent graph set store)

               (if (gethash (list parent child) set)
                   (incf store))

               (loop
                 for n in (gethash child graph)
                 when (/= n parent)
                   do (incf store (dfs n child graph set 0)))
               
               store
               ))
      
      (dfs 0 -1 graph set 0)
      )))

(assert (= 3 (min-reorder 6 '((0 1) (1 3) (2 3) (4 0) (4 5)))))
(assert (= 2 (min-reorder 5 '((1 0) (1 2) (3 2) (3 4)))))
