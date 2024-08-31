(defun next-greater-element (nums1 nums2)
  (let ((table (make-hash-table))
        (nums2 (reverse nums2))
        (bucket '()))
    (loop for n in nums2
          do (loop when (null bucket)
                     do (setf (gethash n table) -1)
                     and return nil
                   if (> (car bucket) n)
                     do (setf (gethash n table) (car bucket))
                     and return nil
                   else
                     do (pop bucket)
                   )
          do (push n bucket))

    (mapcar (lambda (n) (gethash n table)) nums1)
    ))

(assert (equal '(-1 3 -1) (next-greater-element '(4 1 2) '(1 3 4 2))))
(assert (equal '(3 -1) (next-greater-element '(2 4) '(1 2 3 4))))
